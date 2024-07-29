// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

contract Voting {
    struct Voter {
        bool isRegistered;
        bool hasVoted;
        uint8 vote; // ID of the proposal the voter voted for
    }

    struct Proposal {
        string name;
        uint voteCount; // Number of accumulated votes
    }

    address public owner;
    mapping(address => Voter) public voters;
    Proposal[] public proposals;

    // Constructor to initialize the contract with the proposal names
    constructor(string[] memory proposalNames) {
        owner = msg.sender;
        voters[owner].isRegistered = true;

        for (uint i = 0; i < proposalNames.length; i++) {
            proposals.push(Proposal({
                name: proposalNames[i],
                voteCount: 0
            }));
        }
    }

    // Function to register a new voter
    function register(address voter) public {
        require(
            msg.sender == owner,
            "Only owner can register a voter."
        );
        require(
            !voters[voter].isRegistered,
            "The voter is already registered."
        );
        voters[voter].isRegistered = true;
    }

    // Function to vote for a proposal
    function vote(uint8 proposal) public {
        Voter storage sender = voters[msg.sender];
        require(sender.isRegistered, "You are not registered to vote.");
        require(!sender.hasVoted, "You have already voted.");
        require(proposal < proposals.length, "Invalid proposal.");

        sender.hasVoted = true;
        sender.vote = proposal;

        // Increase the vote count of the selected proposal
        proposals[proposal].voteCount += 1;
    }

    // Function to get the results of the voting
    function getResults() public view returns (string memory winnerName, uint winnerVoteCount) {
        uint winningVoteCount = 0;
        uint winningProposalIndex = 0;

        for (uint i = 0; i < proposals.length; i++) {
            if (proposals[i].voteCount > winningVoteCount) {
                winningVoteCount = proposals[i].voteCount;
                winningProposalIndex = i;
            }
        }

        winnerName = proposals[winningProposalIndex].name;
        winnerVoteCount = proposals[winningProposalIndex].voteCount;
    }

    // Function to get proposal details by index
    function getProposal(uint index) public view returns (string memory name, uint voteCount) {
        require(index < proposals.length, "Invalid proposal index.");
        Proposal storage proposal = proposals[index];
        return (proposal.name, proposal.voteCount);
    }
}
