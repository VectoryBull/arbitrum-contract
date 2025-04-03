CONTRACT=0x8e1308925a26cb5cf400afb402d67b3523473379

cast send --rpc-url 'http://localhost:8547' --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
$CONTRACT "init(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)" 0 10 0 10 10 10 2753 3233

cast send --rpc-url 'http://localhost:8547' --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
$CONTRACT "storeSensorData(uint256,uint256,uint256,uint256,uint256,uint256)" 1 1 1 1 1 3086

cast send --rpc-url 'http://localhost:8547' --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
$CONTRACT "storeSensorData(uint256,uint256,uint256,uint256,uint256,uint256)" 1 1 1 1 1 3

cast call --rpc-url 'http://localhost:8547' \
--private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
$CONTRACT "printTemp(uint256)(uint256)" 0 

cast storage $CONTRACT 0 --rpc-url 'http://localhost:8547'

cast send --rpc-url 'http://localhost:8547' --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
$CONTRACT "terminateDataCollection()"

cast call --rpc-url 'http://localhost:8547' \
$CONTRACT "printFinalState()(uint256)" \
--private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659