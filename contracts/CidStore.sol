pragma solidity 0.5.17;

contract Counter {

    // Private variable of type string to store the CID
    string cid = "";

    // Function that stores a given CID
    function setCid(string memory s) public {
        cid = s;
    }

    // Getter function to access the stored CID.
    function getCid() public view returns (string memory) {
        return cid;
    }

}
