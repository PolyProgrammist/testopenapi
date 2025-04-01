import json
import re

def iterate_nested_json_for_loop(json_obj):
    if 'allOf' in json_obj and '$ref' in json_obj['allOf'][0] and json_obj['allOf'][0]['$ref'] == "#/components/schemas/Rational32SchemaProvider" and 'default' in json_obj and not isinstance(json_obj['default'], dict):
        json_obj['default'] = {
            'denom': json_obj['default'][0],
            'numer': json_obj['default'][1]
        }
    for key, value in json_obj.items():
        if isinstance(value, dict):
            iterate_nested_json_for_loop(value)

filename = '../testokplain/transaction.json'
f = open(filename)
filedata = f.read()
f.close()

filedata = filedata.replace('"type": "null"', '"type": "object"')
# filedata = filedata.replace('"nanos"', '"nanoseconds"')
# filedata = filedata.replace('"secs"', '"seconds"')

f = open(filename, 'w')
f.write(filedata)
f.close()


f = open(filename, 'r')
spec = json.load(f)
f.close()

iterate_nested_json_for_loop(spec)

if 'JsonRpcResponse_for_Array_of_Tuple_of_uint64_and_uint64_and_RpcError' in spec['components']['schemas']:
    spec['components']['schemas']['JsonRpcResponse_for_Array_of_Tuple_of_uint64_and_uint64_and_RpcError']['anyOf'][0]['properties']['result']['items']['items'] = {
        "type": "object",
        "properties": {
            "start": {
                "type": "integer",
                "format": "uint64",
                "minimum": 0.0
            },
            "finish": {
                "type": "integer",
                "format": "uint64",
                "minimum": 0.0
            }   
        }
        
    }
   


f = open(filename, 'w')
json.dump(spec, f, indent=4)
f.close()


filename = './keeper/src/lib.rs'
f = open(filename, 'r')
filedata = f.read()
f.close()

newfiledata = re.sub('"{}/\w*', '"{}/', filedata)

f = open(filename, 'w')
f.write(newfiledata)
f.close()


# cd testokplain && cargo run > transaction.json && cd ../progenitor && python3 tx.py && cargo progenitor -i ../testokplain/transaction.json -o keeper -n keeper -v 0.1.0 && python3 tx.py && cd user && cargo run && cd ../..