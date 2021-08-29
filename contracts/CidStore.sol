pragma solidity 0.5.17;

contract Counter {
    
    // Public variable of type unsigned int to keep the number of counts
    string cid = "";

    // Function that increments our counter
    function setCid(string memory s) public {
        cid = s;
    }
    
    // Not necessary getter to get the count value
    function getCid() public view returns (string memory) {
        return cid;
    }

}
