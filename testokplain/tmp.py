import json

a = json.load(open("transaction.json"))

for key, value in a['components']['schemas'].items():
    if key.endswith('View'):
        print(key)