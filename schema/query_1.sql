SELECT 
    Validators.Validator_ID,
    Validators.Validator_Name,
    COUNT(Sync_Committee_Contributions.Contribution_ID) AS num_of_contributions,
    
    (SELECT COUNT(*) FROM Blocks WHERE Proposer_Index = Validators.Validator_ID) AS num_of_slots,

    IFNULL(COUNT(Sync_Committee_Contributions.Contribution_ID) / (SELECT COUNT(*) FROM Blocks WHERE Proposer_Index = Validators.Validator_ID), 0) * 100 AS participation_rate
FROM 
    Validators 
LEFT JOIN 
    Sync_Committee_Contributions ON Validators.Validator_ID = Sync_Committee_Contributions.Subcommittee_Index
GROUP BY 
    Validators.Validator_ID;
