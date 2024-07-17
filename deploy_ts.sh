#!/bin/bash
set -ex

r=${r:-lb1}
dir=$(basename $(pwd))
bin=${1:-invoke_program}

rsync -avz --exclude-from='.gitignore' ../$dir/ $r:$dir/
scp .env $r:$dir/
# ts-node is installed inside nvm
# ssh $r ts-node $dir/typescripts/$bin.ts
# ssh $r systemctl restart $bin
