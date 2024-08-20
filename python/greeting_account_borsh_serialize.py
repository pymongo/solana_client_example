import solders
import solders.keypair
import solders.pubkey
import solders.rpc
import os
from solana.rpc.api import Client
from borsh_construct import CStruct, U32

program_id=solders.pubkey.Pubkey.from_string('8xubajzX923ZXpUzbyTcXuxy9QcMbrUCosm4H6ZRtTtk')
priv_key=open(f'{os.environ["HOME"]}/.config/solana/id.json').read()
payer=solders.keypair.Keypair.from_json(priv_key)

client = Client("https://api.devnet.solana.com")
key = solders.pubkey.Pubkey.create_with_seed(payer.pubkey(), "hello_rust", program_id)
account = client.get_account_info(key)

schema = CStruct("counter" / U32)
data = schema.parse(account.value.data)
print(data)
