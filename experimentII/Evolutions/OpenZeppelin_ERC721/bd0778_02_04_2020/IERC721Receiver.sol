pragma solidity >=0.5.0 <0.9.0;


contract IERC721Receiver {
    
    function onERC721Received(address operator, address from, uint256 tokenId, bytes memory data)
    public  returns (bytes4);
}
