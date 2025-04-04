import json
import requests

def normalize_refs(obj):
    """
    Recursively normalizes `allOf` structures containing a single `$ref` 
    into a direct `$ref`, and merges any other fields in `allOf` into it.
    Also removes `description` fields.
    """
    if isinstance(obj, dict):
        if 'allOf' in obj and len(obj['allOf']) == 1 and '$ref' in obj['allOf'][0]:
            return {
                '$ref': obj['allOf'][0]['$ref'],
                **{k: v for k, v in obj.items() if k != 'allOf' and k != 'description'}
            }
        return { k: v for k, v in obj.items() if k != 'description'}
    elif isinstance(obj, list):
        # Recursively apply to all list elements
        return [normalize_refs(item) for item in obj]
    return obj

def compare_jsons(json1, json2, depth=0, path="root"):
    """
    Recursively compare json1 and json2:
    - At the top level (depth=0), json2 can have extra fields.
    - At deeper levels, json2 cannot have extra fields.
    - Lists are compared after sorting (if they contain sortable elements).
    - Normalizes `#/components/schemas/...` references for consistency.
    - Ignores `description` fields completely.
    
    Prints a human-readable explanation of the differences.
    """
    
    # Normalize references and remove "description" fields before comparison
    json1 = normalize_refs(json1)
    json2 = normalize_refs(json2)
    
    # If both values are lists, compare them sorted
    if isinstance(json1, list) and isinstance(json2, list):
        try:
            if sorted(json1) != sorted(json2):
                return f"Difference found at {path}: Lists do not match."
            return None  # No difference
        except TypeError:
            pass

    # If both values are dictionaries, apply the recursive comparison
    if isinstance(json1, dict) and isinstance(json2, dict):
        if depth == 0:
            filtered_json2 = {key: json2[key] for key in json1 if key in json2}
        else:
            if set(json1.keys()) != set(json2.keys()):
                return f"Difference found at {path}: Keys do not match between the two JSON objects., json1: {json1.keys()}, json2: {json2.keys()}"
            filtered_json2 = json2  # Keep json2 unchanged at deeper levels

        for key in json1:
            new_path = f"{path}.{key}"
            if key not in json2:
                return f"Difference found at {new_path}: Key '{key}' missing in json2., json1: {json1[key]}, json2: {json2.get(key)}"
            diff = compare_jsons(json1[key], json2[key], depth + 1, new_path)
            if diff:
                return diff

        return None  # No difference

    # Direct comparison for other types
    if json1 != json2:
        return f"Difference found at {path}: '{json1}' != '{json2}'"
    
    return None  # No difference

# Step 1: Download the remote JSON
url = "https://raw.githubusercontent.com/PolyProgrammist/testopenapi/master/testokplain/transaction.json"
response = requests.get(url)
remote_json = response.json()

# Step 2: Load the local JSON
with open("transaction.json", "r") as file:
    local_json = json.load(file)

# Step 3: Extract `components.schemas`
local_schemas = local_json.get("components", {}).get("schemas", {})
remote_schemas = remote_json.get("components", {}).get("schemas", {})

# Step 4: Compare the `components.schemas` sections
difference = compare_jsons(local_schemas, remote_schemas)

if difference:
    print(f"JSONs are different: {difference}")
else:
    print("The components.schemas sections are equal.")
