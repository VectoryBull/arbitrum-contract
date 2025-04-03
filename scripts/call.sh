CONTRACT=0xd9bf5428c4a93aa2dedd0161f299071b9d1fec0a

cast send --rpc-url 'http://localhost:8547' --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
$CONTRACT "init(uint256,uint256,uint256,uint256,uint256,uint256)" 1 2 3 4 5 6

cast send --rpc-url 'http://localhost:8547' --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
$CONTRACT "storeSensorData(uint256,uint256,uint256,uint256,uint256)" 1 2 3 4 5

cast call --rpc-url 'http://localhost:8547' \
$CONTRACT "printTemp(uint256)(uint256)" 0 \
--private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \

cast storage $CONTRACT 0 --rpc-url 'http://localhost:8547'