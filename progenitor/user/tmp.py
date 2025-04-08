import json

a = b"{\"jsonrpc\":\"2.0\",\"result\":{\"block_hash\":\"Dxhrj21NWZYKi3DpCtQNtmhLj5sg6FwVVQCRn3EyLZLF\",\"changes\":[{\"cause\":{\"receipt_hash\":\"5LJ4MWb8kqKNjAMdgGhyRA2DYwtXAkh2kueTjjygMY3a\",\"type\":\"receipt_processing\"},\"change\":{\"account_id\":\"token.sweat\",\"amount\":\"30448538655560442306571876737\",\"code_hash\":\"BtMsJANH2hosyCshEL3RrnNw4K1XWjTsUzwiucmhbCit\",\"locked\":\"0\",\"storage_paid_at\":0,\"storage_usage\":2869014130},\"type\":\"account_update\"},{\"cause\":{\"receipt_hash\":\"5LJ4MWb8kqKNjAMdgGhyRA2DYwtXAkh2kueTjjygMY3a\",\"type\":\"action_receipt_gas_reward\"},\"change\":{\"account_id\":\"token.sweat\",\"amount\":\"30448538762159824426271876737\",\"code_hash\":\"BtMsJANH2hosyCshEL3RrnNw4K1XWjTsUzwiucmhbCit\",\"locked\":\"0\",\"storage_paid_at\":0,\"storage_usage\":2869014130},\"type\":\"account_update\"}]},\"id\":\"dontcare\"}"
x = a.decode('utf8')
t = json.loads(x)
print(json.dumps(t['result']))