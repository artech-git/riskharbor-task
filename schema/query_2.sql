

SELECT a.Validator_ID, a.Epoch, COUNT(*) AS MissedAttestations
FROM Attestations AS a
LEFT JOIN Aggregate_Attestations AS ag ON a.Block_ID = ag.Attestation_Data_Root
WHERE a.Epoch >= (SELECT MAX(Epoch) - 4 FROM Attestations) -- Retrieve the 5 most recent epochs
AND ag.Attestation_Data_Root IS NULL -- No corresponding entry in Aggregate_Attestations
GROUP BY a.Validator_ID, a.Epoch;
