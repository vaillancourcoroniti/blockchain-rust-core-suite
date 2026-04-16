use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    ParameterChange,
    FundAllocation,
    Upgrade,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub proposer: String,
    pub start_time: u128,
    pub end_time: u128,
    pub yes_votes: u128,
    pub no_votes: u128,
    pub abstain_votes: u128,
    pub executed: bool,
}

pub struct GovernanceVoting {
    proposals: HashMap<String, GovernanceProposal>,
    votes: HashMap<(String, String), VoteChoice>,
    min_proposal_deposit: u128,
    voting_period_ms: u128,
}

impl GovernanceVoting {
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            votes: HashMap::new(),
            min_proposal_deposit: 1000,
            voting_period_ms: 7 * 24 * 3600 * 1000,
        }
    }

    pub fn create_proposal(&mut self, mut proposal: GovernanceProposal) -> Result<(), String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        proposal.start_time = now;
        proposal.end_time = now + self.voting_period_ms;
        self.proposals.insert(proposal.id.clone(), proposal);
        Ok(())
    }

    pub fn cast_vote(&mut self, proposal_id: &str, voter: String, choice: VoteChoice, voting_power: u128) -> Result<(), String> {
        let proposal = self.proposals.get_mut(proposal_id).ok_or("Proposal not found")?;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        if now < proposal.start_time || now > proposal.end_time {
            return Err("Voting not active".to_string());
        }
        
        if self.votes.contains_key(&(proposal_id.to_string(), voter.clone())) {
            return Err("Already voted".to_string());
        }
        
        self.votes.insert((proposal_id.to_string(), voter), choice.clone());
        
        match choice {
            VoteChoice::Yes => proposal.yes_votes += voting_power,
            VoteChoice::No => proposal.no_votes += voting_power,
            VoteChoice::Abstain => proposal.abstain_votes += voting_power,
        }
        
        Ok(())
    }

    pub fn execute_proposal(&mut self, id: &str) -> Result<(), String> {
        let proposal = self.proposals.get_mut(id).ok_or("Proposal not found")?;
        if proposal.executed {
            return Err("Already executed".to_string());
        }
        proposal.executed = true;
        Ok(())
    }

    pub fn get_proposal(&self, id: &str) -> Option<&GovernanceProposal> {
        self.proposals.get(id)
    }
}
