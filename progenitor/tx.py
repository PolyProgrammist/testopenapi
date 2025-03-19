import json
import re

filename = '../testokplain/transaction.json'
f = open(filename)
filedata = f.read()
f.close()

filedata = filedata.replace('"type": "null"', '"type": "object"')

f = open(filename, 'w')
f.write(filedata)
f.close()

# spec = json.load(f)
# f.close()
# if 'StorageError' in spec['components']['schemas']:
#     spec['components']['schemas']['StorageError']['oneOf'][1]['properties']['MissingTrieValue']['items'] = {"oneOf": spec['components']['schemas']['StorageError']['oneOf'][1]['properties']['MissingTrieValue']['items']}
# json.dump(spec, open(filename, 'w'), indent = 4)

filename = './keeper/src/lib.rs'
f = open(filename, 'r')
filedata = f.read()
f.close()

newfiledata = re.sub('"{}/\w*', '"{}/', filedata)

f = open(filename, 'w')
f.write(newfiledata)
f.close()


# cd testokplain && cargo run > transaction.json && cd ../progenitor && python3 tx.py && cargo progenitor -i ../testokplain/transaction.json -o keeper -n keeper -v 0.1.0 && python3 tx.py && cd user && cargo run && cd ../..