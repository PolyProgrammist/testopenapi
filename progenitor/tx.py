import json
filename = '../testokplain/transaction.json'
f = open(filename)
spec = json.load(f)
f.close()
spec['components']['schemas']['StorageError']['oneOf'][1]['properties']['MissingTrieValue'] = {"type": "string"}
json.dump(spec, open(filename, 'w'), indent = 4)


# cd testokplain && cargo run > transaction.json && cd ../progenitor && python3 tx.py && cargo progenitor -i ../testokplain/transaction.json -o keeper -n keeper -v 0.1.0 && cd user && cargo run && cd ../..