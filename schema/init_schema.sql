CREATE TABLE Validators (
    Validator_ID INT PRIMARY KEY,
    Validator_Name VARCHAR(255),
    PublicKey VARCHAR(255),
    Balance FLOAT,
    Status VARCHAR(50),
    Activation_Epoch INT,
    Exit_Epoch INT,
    Slashed BOOLEAN
);

CREATE TABLE Blocks (
    Block_ID VARCHAR(255) PRIMARY KEY,
    Slot INT,
    Proposer_Index INT,
    Parent_Root VARCHAR(255),
    State_Root VARCHAR(255),
    Body_Root VARCHAR(255),
    Signature VARCHAR(255),
    FOREIGN KEY (Proposer_Index) REFERENCES Validators(Validator_ID)
);

CREATE TABLE Attestations (
    Attestation_ID INT AUTO_INCREMENT PRIMARY KEY,
    Block_ID VARCHAR(255),
    Aggregation_Bits VARCHAR(255),
    Data VARCHAR(255),
    Slot INT,
    Index INT,
    Beacon_Block_Root VARCHAR(255),
    Source VARCHAR(255),
    Epoch INT,
    Root VARCHAR(255),
    FOREIGN KEY (Block_ID) REFERENCES Blocks(Block_ID)
);

CREATE TABLE Aggregate_Attestations (
    Aggregate_Attestation_ID INT AUTO_INCREMENT PRIMARY KEY,
    Attestation_Data_Root VARCHAR(255),
    Slot INT,
    Aggregation_Bits VARCHAR(255),
    Signature VARCHAR(255),
    Data VARCHAR(255),
    Index INT,
    Beacon_Block_Root VARCHAR(255),
    Source VARCHAR(255),
    Epoch INT,
    Root VARCHAR(255)
);

CREATE TABLE Sync_Committee_Contributions (
    Contribution_ID INT AUTO_INCREMENT PRIMARY KEY,
    Slot INT,
    Subcommittee_Index INT,
    Beacon_Block_Root VARCHAR(255),
    Aggregation_Bits VARCHAR(255),
    Signature VARCHAR(255)
);
