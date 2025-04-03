CONTRACT=0x408da76e87511429485c32e4ad647dd14823fdc4

cast send --rpc-url 'http://localhost:8547' --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
$CONTRACT "init(int256,int256,int256,int256,int256)" 1 2 3 4 5 6

cast send --rpc-url 'http://localhost:8547' --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
$CONTRACT "storeSensorData(int256,int256,int256,int256,int256)" 1 2 3 4 5

cast call --rpc-url 'http://localhost:8547' \
$CONTRACT "printTemp(uint256)(int256)" 0 \
--private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \

cast storage $CONTRACT 0 --rpc-url 'http://localhost:8547'