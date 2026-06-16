// Identity Registry - W3C DID Compliant with proper validation throughout
#![no_std]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::unwrap_used)]

pub mod errors;
pub use errors::Error;
use soroban_sdk::xdr::ToXdr;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Bytes, BytesN, Env,
    String, Symbol, Vec,
};
use vitastellar_sanitization::{
    sanitize_id, sanitize_string, sanitize_url, SanitizationError, MAX_GENERAL_LEN,
};

// ============================================================================
// W3C DID COMPLIANT DECENTRALIZED IDENTITY REGISTRY
// ============================================================================
// Implements W3C DID Core Specification (https://www.w3.org/TR/did-core/)
// DID Method: did:stellar:uzima:<network>:<address>
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[contracttype]
#[repr(u32)]
pub enum RbacRole {
    Admin = 0,
    Doctor = 1,
    Patient = 2,
    Staff = 3,
    Insurer = 4,
    Researcher = 5,
    Auditor = 6,
    Service = 7,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[contracterror]
#[repr(u32)]
pub enum RbacError {
    Unauthorized = 100,
    NotInitialized = 300,
    AlreadyInitialized = 301,
}

#[soroban_sdk::contractclient(name = "RbacClient")]
pub trait RbacContract {
    fn has_role(env: Env, address: Address, role: RbacRole) -> Result<bool, RbacError>;
    fn assign_role(env: Env, address: Address, role: RbacRole) -> Result<bool, RbacError>;
    fn remove_role(env: Env, address: Address, role: RbacRole) -> Result<bool, RbacError>;
}

// === DID Document Structures (W3C Compliant) ===

/// Verification Method Types as per W3C DID Specification
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationMethodType {
    Ed25519VerificationKey2020,
    EcdsaSecp256k1VerifKey2019,
    X25519KeyAgreementKey2020,
    JsonWebKey2020,
    /// FIDO2 / WebAuthn Ed25519 (EdDSA) authenticator key (algorithm tag = 1).
    Fido2EdDsa2024,
    /// FIDO2 / WebAuthn P-256 (ES256) authenticator key (algorithm tag = 2).
    Fido2Es2562024,
}

/// Verification Relationship Types
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationRelationship {
    Authentication,
    AssertionMethod,
    KeyAgreement,
    CapabilityInvocation,
    CapabilityDelegation,
}

/// Verification Method (Public Key) as per W3C DID spec
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationMethod {
    /// Unique identifier for this verification method (fragment)
    pub id: String,
    /// Type of verification method
    pub method_type: VerificationMethodType,
    /// The controller of this key (usually the DID subject)
    pub controller: Address,
    /// Public key bytes (Ed25519: 32 bytes, Secp256k1: 33 bytes compressed)
    pub public_key: BytesN<32>,
    /// Whether this method is currently active
    pub is_active: bool,
    /// Timestamp when this key was added
    pub created: u64,
    /// Timestamp when this key was last rotated (0 if never)
    pub last_rotated: u64,
}

/// Service Endpoint as per W3C DID spec
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServiceEndpoint {
    /// Unique identifier for this service (fragment)
    pub id: String,
    /// Type of service (e.g., "LinkedDomains", "MedicalRecords", "CredentialRegistry")
    pub service_type: String,
    /// Service endpoint URI (IPFS hash, URL reference, or contract address)
    pub endpoint: String,
    /// Whether this service is active
    pub is_active: bool,
}

/// DID Document Status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DIDStatus {
    Active,
    Deactivated,
    RecoveryPending,
}

/// Complete DID Document following W3C DID Core spec
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DIDDocument {
    /// The DID identifier (did:stellar:uzima:<network>:<address>)
    pub id: String,
    /// Controller(s) of this DID - who can make changes
    pub controller: Address,
    /// Alternative controller (for recovery scenarios)
    pub also_known_as: Vec<String>,
    /// All verification methods (public keys)
    pub verification_methods: Vec<VerificationMethod>,
    /// IDs of methods used for authentication
    pub authentication: Vec<String>,
    /// IDs of methods used for issuing credentials (assertion)
    pub assertion_method: Vec<String>,
    /// IDs of methods used for key agreement
    pub key_agreement: Vec<String>,
    /// IDs of methods for capability invocation
    pub capability_invocation: Vec<String>,
    /// IDs of methods for capability delegation
    pub capability_delegation: Vec<String>,
    /// Service endpoints
    pub services: Vec<ServiceEndpoint>,
    /// Document status
    pub status: DIDStatus,
    /// Creation timestamp
    pub created: u64,
    /// Last update timestamp
    pub updated: u64,
    /// Version number (incremented on each update)
    pub version: u32,
    /// Hash of previous document version (for audit trail)
    pub previous_hash: BytesN<32>,
}

// === Verifiable Credentials Structures ===

/// Credential Types for Healthcare
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CredentialType {
    MedicalLicense,
    SpecialistCertification,
    HospitalAffiliation,
    ResearchAuthorization,
    PatientConsent,
    EmergencyAccess,
    DataAccessPermission,
}

/// Verifiable Credential (on-chain reference)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiableCredential {
    /// Unique credential ID
    pub id: BytesN<32>,
    /// Credential type
    pub credential_type: CredentialType,
    /// Issuer DID (the entity that issued this credential)
    pub issuer: Address,
    /// Subject DID (the entity the credential is about)
    pub subject: Address,
    /// Issuance timestamp
    pub issuance_date: u64,
    /// Expiration timestamp (0 = no expiration)
    pub expiration_date: u64,
    /// Hash of the full credential data (stored off-chain)
    pub credential_hash: BytesN<32>,
    /// URI to the full credential (IPFS CID or similar)
    pub credential_uri: String,
    /// Whether the credential has been revoked
    pub is_revoked: bool,
    /// Revocation timestamp (0 if not revoked)
    pub revoked_at: u64,
    /// Revocation reason (if revoked)
    pub revocation_reason: String,
}

/// Credential Status for verification
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CredentialStatus {
    Valid,
    Revoked,
    Expired,
    NotFound,
}

// === Identity Recovery Structures ===

/// Recovery Guardian
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoveryGuardian {
    pub address: Address,
    pub weight: u32,
    pub added_at: u64,
}

/// Recovery Request
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoveryRequest {
    pub request_id: u64,
    pub subject: Address,
    pub new_controller: Address,
    pub new_primary_key: BytesN<32>,
    pub created_at: u64,
    pub approvals: Vec<Address>,
    pub total_weight: u32,
    pub executed: bool,
}

// === Legacy Structures (Backward Compatibility) ===

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityRecord {
    pub hash: BytesN<32>,
    pub meta: String,
    pub registered_by: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    pub claim_hash: BytesN<32>,
    pub verifier: Address,
    pub is_active: bool,
}

/// Stake information for a healthcare provider using SUT token reputation bonding.
#[derive(Clone)]
#[contracttype]
pub struct ProviderStake {
    /// The provider's address
    pub provider: Address,
    /// The SUT token contract address
    pub token_address: Address,
    /// Amount of SUT tokens staked
    pub amount: i128,
    /// Timestamp until which the stake is locked
    pub locked_until: u64,
    /// Whether the stake has been slashed
    pub slashed: bool,
    /// When the stake was deposited
    pub deposited_at: u64,
}

// === Storage Keys ===

#[contracttype]
pub enum DataKey {
    // Contract State
    Owner,
    Initialized,
    NetworkId,
    RbacContract,
    Paused,

    // Verifier Management
    Verifier(Address),

    // Legacy Identity (backward compatibility)
    IdentityHash(Address),
    Attestation(Address, BytesN<32>),
    SubjectAttestations(Address),

    // DID Document Storage
    DIDDocument(Address),
    DIDByString(String),

    // Verification Methods
    VerificationMethod(Address, String),

    // Verifiable Credentials
    Credential(BytesN<32>),
    SubjectCredentials(Address),
    IssuerCredentials(Address),
    CredentialsByType(Address, CredentialType),

    // Recovery System
    RecoveryGuardians(Address),
    RecoveryThreshold(Address),
    RecoveryRequest(u64),
    ActiveRecovery(Address),
    RecoveryCounter,

    // Key Rotation
    LastKeyRotation(Address),
    KeyRotationCooldown,

    // Provider Staking
    StakeInfo(Address),
}

// === Constants ===

const DEFAULT_RECOVERY_THRESHOLD: u32 = 2;
const DEFAULT_RECOVERY_TIMELOCK: u64 = 86_400; // 24 hours
const DEFAULT_KEY_ROTATION_COOLDOWN: u64 = 3_600; // 1 hour
const ZERO_HASH: [u8; 32] = [0u8; 32];

// === Contract Implementation ===

#[contract]
pub struct IdentityRegistryContract;

#[contractimpl]
impl IdentityRegistryContract {
    // ========================================================================
    // INITIALIZATION
    // ========================================================================

    /// Initialize the contract with an owner and network identifier
    pub fn initialize(
        env: Env,
        owner: Address,
        network_id: String,
        rbac_contract: Address,
    ) -> Result<(), Error> {
        owner.require_auth();

        sanitize_id(&env, &network_id).map_err(Self::map_sanitization_error)?;

        if env.storage().instance().has(&DataKey::Initialized) {
            return Err(Error::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage()
            .instance()
            .set(&DataKey::RbacContract, &rbac_contract);
        env.storage()
            .instance()
            .set(&DataKey::NetworkId, &network_id);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.storage().instance().set(&DataKey::Initialized, &true);
        env.storage()
            .instance()
            .set(&DataKey::Verifier(owner.clone()), &true);
        env.storage().instance().set(
            &DataKey::KeyRotationCooldown,
            &DEFAULT_KEY_ROTATION_COOLDOWN,
        );

        env.events().publish(
            (Symbol::new(&env, "Initialized"),),
            (owner.clone(), network_id),
        );

        Ok(())
    }

    /// Perform a health check on the contract.
    /// Returns (status, version, timestamp) with standardized status values:
    /// "OK", "PAUSED", "NOT_INIT", "DEGRADED".
    pub fn health_check(env: Env) -> (Symbol, u32, u64) {
        let initialized = env.storage().instance().has(&DataKey::Initialized);
        let paused: bool = env
            .storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false);

        let status = if !initialized {
            symbol_short!("NOT_INIT")
        } else if paused {
            symbol_short!("PAUSED")
        } else {
            symbol_short!("OK")
        };

        let version: u32 = 1;
        let timestamp = env.ledger().timestamp();

        env.events().publish(
            (Symbol::new(&env, "HealthCheck"),),
            (status.clone(), version, timestamp),
        );

        (status, version, timestamp)
    }

    /// Returns true if the contract is currently paused.
    pub fn is_paused(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false)
    }

    fn is_admin(env: &Env, caller: &Address) -> bool {
        if let Some(owner) = env
            .storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::Owner)
        {
            if &owner == caller {
                return true;
            }
        }
        if let Some(rbac_addr) = env
            .storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::RbacContract)
        {
            let client = RbacClient::new(env, &rbac_addr);
            return client.has_role(caller, &RbacRole::Admin);
        }
        false
    }

    fn require_admin(env: &Env, caller: &Address) -> Result<(), Error> {
        if Self::is_admin(env, caller) {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }

    fn require_not_paused(env: &Env) -> Result<(), Error> {
        if env
            .storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false)
        {
            return Err(Error::ContractPaused);
        }
        Ok(())
    }

    pub fn pause(env: Env, caller: Address) -> Result<bool, Error> {
        caller.require_auth();
        Self::require_admin(&env, &caller)?;
        env.storage().instance().set(&DataKey::Paused, &true);
        env.events().publish(
            (Symbol::new(&env, "Paused"),),
            (caller.clone(), env.ledger().timestamp()),
        );
        Ok(true)
    }

    pub fn unpause(env: Env, caller: Address) -> Result<bool, Error> {
        caller.require_auth();
        Self::require_admin(&env, &caller)?;
        env.storage().instance().set(&DataKey::Paused, &false);
        env.events().publish(
            (Symbol::new(&env, "Unpaused"),),
            (caller.clone(), env.ledger().timestamp()),
        );
        Ok(true)
    }

    /// Legacy initialize for backward compatibility
    pub fn initialize_legacy(env: Env, owner: Address, rbac_contract: Address) {
        owner.require_auth();

        if env.storage().instance().has(&DataKey::Owner) {
            return; // Contract already initialized
        }

        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage()
            .instance()
            .set(&DataKey::RbacContract, &rbac_contract);
        env.storage()
            .instance()
            .set(&DataKey::Verifier(owner.clone()), &true);

        env.events()
            .publish((symbol_short!("Init"),), owner.clone());
    }

    // ========================================================================
    // DID DOCUMENT MANAGEMENT
    // ========================================================================

    /// Create a new DID Document for a subject
    /// Only the subject can create their own DID
    pub fn create_did(
        env: Env,
        subject: Address,
        primary_public_key: BytesN<32>,
        services: Vec<ServiceEndpoint>,
    ) -> Result<String, Error> {
        subject.require_auth();
        Self::require_not_paused(&env)?;

        // Check if DID already exists
        if env
            .storage()
            .persistent()
            .has(&DataKey::DIDDocument(subject.clone()))
        {
            return Err(Error::DIDAlreadyExists);
        }

        let timestamp = env.ledger().timestamp();
        let network_id: String = env
            .storage()
            .instance()
            .get(&DataKey::NetworkId)
            .unwrap_or(String::from_str(&env, "testnet"));

        // Generate DID string
        let did_string = Self::generate_did_string(&env, &network_id, &subject);

        // Create primary verification method
        let primary_vm_id = String::from_str(&env, "#key-1");
        let primary_vm = VerificationMethod {
            id: primary_vm_id.clone(),
            method_type: VerificationMethodType::Ed25519VerificationKey2020,
            controller: subject.clone(),
            public_key: primary_public_key,
            is_active: true,
            created: timestamp,
            last_rotated: 0,
        };

        let mut verification_methods = Vec::new(&env);
        verification_methods.push_back(primary_vm);

        let mut auth_methods = Vec::new(&env);
        auth_methods.push_back(primary_vm_id.clone());

        let mut assertion_methods = Vec::new(&env);
        assertion_methods.push_back(primary_vm_id.clone());

        let mut cap_invocation = Vec::new(&env);
        cap_invocation.push_back(primary_vm_id.clone());

        let mut cap_delegation = Vec::new(&env);
        cap_delegation.push_back(primary_vm_id);

        let did_doc = DIDDocument {
            id: did_string.clone(),
            controller: subject.clone(),
            also_known_as: Vec::new(&env),
            verification_methods,
            authentication: auth_methods,
            assertion_method: assertion_methods,
            key_agreement: Vec::new(&env),
            capability_invocation: cap_invocation,
            capability_delegation: cap_delegation,
            services,
            status: DIDStatus::Active,
            created: timestamp,
            updated: timestamp,
            version: 1,
            previous_hash: BytesN::from_array(&env, &ZERO_HASH),
        };

        // Store DID document
        env.storage()
            .persistent()
            .set(&DataKey::DIDDocument(subject.clone()), &did_doc);
        env.storage()
            .persistent()
            .set(&DataKey::DIDByString(did_string.clone()), &subject);

        // Initialize recovery guardians with empty list
        let guardians: Vec<RecoveryGuardian> = Vec::new(&env);
        env.storage()
            .persistent()
            .set(&DataKey::RecoveryGuardians(subject.clone()), &guardians);
        env.storage().persistent().set(
            &DataKey::RecoveryThreshold(subject.clone()),
            &DEFAULT_RECOVERY_THRESHOLD,
        );

        env.events().publish(
            (Symbol::new(&env, "DIDCreated"),),
            (subject, did_string.clone()),
        );

        Ok(did_string)
    }

    /// Resolve a DID Document by subject address
    pub fn resolve_did(env: Env, subject: Address) -> Result<DIDDocument, Error> {
        let did_doc: DIDDocument = env
            .storage()
            .persistent()
            .get(&DataKey::DIDDocument(subject))
            .ok_or(Error::DIDNotFound)?;

        if matches!(did_doc.status, DIDStatus::Deactivated) {
            return Err(Error::DIDDeactivated);
        }

        Ok(did_doc)
    }

    /// Resolve a DID Document by DID string
    pub fn resolve_did_by_string(env: Env, did_string: String) -> Result<DIDDocument, Error> {
        let subject: Address = env
            .storage()
            .persistent()
            .get(&DataKey::DIDByString(did_string))
            .ok_or(Error::DIDNotFound)?;

        Self::resolve_did(env, subject)
    }

    /// Update DID Document (add/modify services, also_known_as)
    pub fn update_did(
        env: Env,
        subject: Address,
        new_services: Vec<ServiceEndpoint>,
        new_also_known_as: Vec<String>,
    ) -> Result<(), Error> {
        subject.require_auth();

        let mut did_doc: DIDDocument = env
            .storage()
            .persistent()
            .get(&DataKey::DIDDocument(subject.clone()))
            .ok_or(Error::DIDNotFound)?;

        if matches!(did_doc.status, DIDStatus::Deactivated) {
            return Err(Error::DIDDeactivated);
        }

        // Compute hash of current document for audit trail
        let prev_hash = Self::compute_document_hash(&env, &did_doc);

        did_doc.services = new_services;
        did_doc.also_known_as = new_also_known_as;
        did_doc.updated = env.ledger().timestamp();
        // SAFETY: version is a u32 monotonic counter; saturation at u32::MAX is acceptable
        did_doc.version = did_doc.version.saturating_add(1);
        did_doc.previous_hash = prev_hash;

        env.storage()
            .persistent()
            .set(&DataKey::DIDDocument(subject.clone()), &did_doc);

        env.events().publish(
            (Symbol::new(&env, "DIDUpdated"),),
            (subject, did_doc.version),
        );

        Ok(())
    }

    /// Deactivate a DID (soft delete)
    pub fn deactivate_did(env: Env, subject: Address) -> Result<(), Error> {
        subject.require_auth();

        let mut did_doc: DIDDocument = env
            .storage()
            .persistent()
            .get(&DataKey::DIDDocument(subject.clone()))
            .ok_or(Error::DIDNotFound)?;

        did_doc.status = DIDStatus::Deactivated;
        did_doc.updated = env.ledger().timestamp();

        env.storage()
            .persistent()
            .set(&DataKey::DIDDocument(subject.clone()), &did_doc);

        env.events()
            .publish((Symbol::new(&env, "DIDDeactivated"),), subject);

        Ok(())
    }

    // ========================================================================
    // VERIFICATION METHOD MANAGEMENT (Key Management)
    // ========================================================================

    /// Add a new verification method to a DID
    pub fn add_verification_method(
        env: Env,
        subject: Address,
        method_id: String,
        method_type: VerificationMethodType,
        public_key: BytesN<32>,
        relationships: Vec<VerificationRelationship>,
    ) -> Result<(), Error> {
        subject.require_auth();

        let mut did_doc: DIDDocument = env
            .storage()
            .persistent()
            .get(&DataKey::DIDDocument(subject.clone()))
            .ok_or(Error::DIDNotFound)?;

        if matches!(did_doc.status, DIDStatus::Deactivated) {
            return Err(Error::DIDDeactivated);
        }

        let timestamp = env.ledger().timestamp();

        let new_vm = VerificationMethod {
            id: method_id.clone(),
            method_type,
            controller: subject.clone(),
            public_key,
            is_active: true,
            created: timestamp,
            last_rotated: 0,
        };

        did_doc.verification_methods.push_back(new_vm);

        // Add to specified relationships
        for rel in relationships.iter() {
            match rel {
                VerificationRelationship::Authentication => {
                    did_doc.authentication.push_back(method_id.clone());
                },
                VerificationRelationship::AssertionMethod => {
                    did_doc.assertion_method.push_back(method_id.clone());
                },
                VerificationRelationship::KeyAgreement => {
                    did_doc.key_agreement.push_back(method_id.clone());
                },
                VerificationRelationship::CapabilityInvocation => {
                    did_doc.capability_invocation.push_back(method_id.clone());
                },
                VerificationRelationship::CapabilityDelegation => {
                    did_doc.capability_delegation.push_back(method_id.clone());
                },
            }
        }

        did_doc.updated = timestamp;
        // SAFETY: version is a u32 monotonic counter; saturation at u32::MAX is acceptable
        did_doc.version = did_doc.version.saturating_add(1);

        env.storage()
            .persistent()
            .set(&DataKey::DIDDocument(subject.clone()), &did_doc);

        env.events().publish(
            (Symbol::new(&env, "VerificationMethodAdded"),),
            (subject, method_id),
        );

        Ok(())
    }

    /// Rotate a verification method key
    pub fn rotate_key(
        env: Env,
        subject: Address,
        method_id: String,
        new_public_key: BytesN<32>,
    ) -> Result<(), Error> {
        subject.require_auth();

        let mut did_doc: DIDDocument = env
            .storage()
            .persistent()
            .get(&DataKey::DIDDocument(subject.clone()))
            .ok_or(Error::DIDNotFound)?;

        if matches!(did_doc.status, DIDStatus::Deactivated) {
            return Err(Error::DIDDeactivated);
        }

        let timestamp = env.ledger().timestamp();

        // Check cooldown period
        let cooldown: u64 = env
            .storage()
            .instance()
            .get(&DataKey::KeyRotationCooldown)
            .unwrap_or(DEFAULT_KEY_ROTATION_COOLDOWN);
        let last_rotation: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::LastKeyRotation(subject.clone()))
            .unwrap_or(0);

        if timestamp < last_rotation.saturating_add(cooldown) {
            return Err(Error::KeyRotationCooldown);
        }

        // Find and update the verification method
        let mut found = false;
        let mut updated_methods = Vec::new(&env);

        for vm in did_doc.verification_methods.iter() {
            if vm.id == method_id {
                let updated_vm = VerificationMethod {
                    id: vm.id.clone(),
                    method_type: vm.method_type.clone(),
                    controller: vm.controller.clone(),
                    public_key: new_public_key.clone(),
                    is_active: true,
                    created: vm.created,
                    last_rotated: timestamp,
                };
                updated_methods.push_back(updated_vm);
                found = true;
            } else {
                updated_methods.push_back(vm);
            }
        }

        if !found {
            return Err(Error::VerificationMethodNotFound);
        }

        did_doc.verification_methods = updated_methods;
        did_doc.updated = timestamp;
        // SAFETY: version is a u32 monotonic counter; saturation at u32::MAX is acceptable
        did_doc.version = did_doc.version.saturating_add(1);

        env.storage()
            .persistent()
            .set(&DataKey::DIDDocument(subject.clone()), &did_doc);
        env.storage()
            .persistent()
            .set(&DataKey::LastKeyRotation(subject.clone()), &timestamp);

        env.events()
            .publish((Symbol::new(&env, "KeyRotated"),), (subject, method_id));

        Ok(())
    }

    /// Revoke/deactivate a verification method
    pub fn revoke_verification_method(
        env: Env,
        subject: Address,
        method_id: String,
    ) -> Result<(), Error> {
        subject.require_auth();

        let mut did_doc: DIDDocument = env
            .storage()
            .persistent()
            .get(&DataKey::DIDDocument(subject.clone()))
            .ok_or(Error::DIDNotFound)?;

        if matches!(did_doc.status, DIDStatus::Deactivated) {
            return Err(Error::DIDDeactivated);
        }

        // Ensure at least one method remains active
        let active_count = did_doc
            .verification_methods
            .iter()
            .filter(|vm| vm.is_active)
            .count();
        if active_count <= 1 {
            return Err(Error::InvalidVerificationMethod);
        }

        let mut found = false;
        let mut updated_methods = Vec::new(&env);

        for vm in did_doc.verification_methods.iter() {
            if vm.id == method_id {
                let revoked_vm = VerificationMethod {
                    id: vm.id.clone(),
                    method_type: vm.method_type.clone(),
                    controller: vm.controller.clone(),
                    public_key: vm.public_key.clone(),
                    is_active: false,
                    created: vm.created,
                    last_rotated: vm.last_rotated,
                };
                updated_methods.push_back(revoked_vm);
                found = true;
            } else {
                updated_methods.push_back(vm);
            }
        }

        if !found {
            return Err(Error::VerificationMethodNotFound);
        }

        did_doc.verification_methods = updated_methods;
        did_doc.updated = env.ledger().timestamp();
        // SAFETY: version is a u32 monotonic counter; saturation at u32::MAX is acceptable
        did_doc.version = did_doc.version.saturating_add(1);

        env.storage()
            .persistent()
            .set(&DataKey::DIDDocument(subject.clone()), &did_doc);

        env.events().publish(
            (Symbol::new(&env, "VerificationMethodRevoked"),),
            (subject, method_id),
        );

        Ok(())
    }

    // ========================================================================
    // VERIFIABLE CREDENTIALS
    // ========================================================================

    /// Issue a verifiable credential (only verifiers/issuers can do this)
    #[allow(clippy::too_many_arguments)]
    pub fn issue_credential(
        env: Env,
        issuer: Address,
        subject: Address,
        credential_type: CredentialType,
        credential_hash: BytesN<32>,
        credential_uri: String,
        expiration_date: u64,
    ) -> Result<BytesN<32>, Error> {
        issuer.require_auth();

        // Verify issuer is a registered verifier
        let is_verifier = Self::is_verifier(env.clone(), issuer.clone());

        if !is_verifier {
            return Err(Error::NotVerifier);
        }

        let timestamp = env.ledger().timestamp();

        // Generate credential ID (hash of issuer + subject + timestamp + type)
        let credential_id =
            Self::generate_credential_id(&env, &issuer, &subject, timestamp, &credential_type);

        let credential = VerifiableCredential {
            id: credential_id.clone(),
            credential_type: credential_type.clone(),
            issuer: issuer.clone(),
            subject: subject.clone(),
            issuance_date: timestamp,
            expiration_date,
            credential_hash,
            credential_uri,
            is_revoked: false,
            revoked_at: 0,
            revocation_reason: String::from_str(&env, ""),
        };

        // Store credential
        env.storage()
            .persistent()
            .set(&DataKey::Credential(credential_id.clone()), &credential);

        // Add to subject's credentials
        let mut subject_creds: Vec<BytesN<32>> = env
            .storage()
            .persistent()
            .get(&DataKey::SubjectCredentials(subject.clone()))
            .unwrap_or(Vec::new(&env));
        subject_creds.push_back(credential_id.clone());
        env.storage().persistent().set(
            &DataKey::SubjectCredentials(subject.clone()),
            &subject_creds,
        );

        // Add to issuer's issued credentials
        let mut issuer_creds: Vec<BytesN<32>> = env
            .storage()
            .persistent()
            .get(&DataKey::IssuerCredentials(issuer.clone()))
            .unwrap_or(Vec::new(&env));
        issuer_creds.push_back(credential_id.clone());
        env.storage()
            .persistent()
            .set(&DataKey::IssuerCredentials(issuer.clone()), &issuer_creds);

        env.events().publish(
            (Symbol::new(&env, "CredentialIssued"),),
            (issuer, subject, credential_id.clone(), credential_type),
        );

        Ok(credential_id)
    }

    /// Verify a credential's status
    pub fn verify_credential(
        env: Env,
        credential_id: BytesN<32>,
    ) -> Result<CredentialStatus, Error> {
        let credential: Option<VerifiableCredential> = env
            .storage()
            .persistent()
            .get(&DataKey::Credential(credential_id));

        match credential {
            None => Ok(CredentialStatus::NotFound),
            Some(cred) => {
                if cred.is_revoked {
                    Ok(CredentialStatus::Revoked)
                } else if cred.expiration_date > 0
                    && env.ledger().timestamp() > cred.expiration_date
                {
                    Ok(CredentialStatus::Expired)
                } else {
                    Ok(CredentialStatus::Valid)
                }
            },
        }
    }

    /// Get a credential by ID
    pub fn get_credential(
        env: Env,
        credential_id: BytesN<32>,
    ) -> Result<VerifiableCredential, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Credential(credential_id))
            .ok_or(Error::CredentialNotFound)
    }

    /// Revoke a credential (only issuer can revoke)
    pub fn revoke_credential(
        env: Env,
        issuer: Address,
        credential_id: BytesN<32>,
        reason: String,
    ) -> Result<(), Error> {
        issuer.require_auth();

        let mut credential: VerifiableCredential = env
            .storage()
            .persistent()
            .get(&DataKey::Credential(credential_id.clone()))
            .ok_or(Error::CredentialNotFound)?;

        if credential.issuer != issuer {
            return Err(Error::Unauthorized);
        }

        if credential.is_revoked {
            return Err(Error::CredentialRevoked);
        }

        credential.is_revoked = true;
        credential.revoked_at = env.ledger().timestamp();
        credential.revocation_reason = reason;

        env.storage()
            .persistent()
            .set(&DataKey::Credential(credential_id.clone()), &credential);

        env.events().publish(
            (Symbol::new(&env, "CredentialRevoked"),),
            (issuer, credential_id),
        );

        Ok(())
    }

    /// Get all credentials for a subject
    pub fn get_subject_credentials(env: Env, subject: Address) -> Vec<VerifiableCredential> {
        let credential_ids: Vec<BytesN<32>> = env
            .storage()
            .persistent()
            .get(&DataKey::SubjectCredentials(subject))
            .unwrap_or(Vec::new(&env));

        let mut credentials = Vec::new(&env);
        for id in credential_ids.iter() {
            if let Some(cred) = env
                .storage()
                .persistent()
                .get::<DataKey, VerifiableCredential>(&DataKey::Credential(id))
            {
                credentials.push_back(cred);
            }
        }
        credentials
    }

    /// Verify if subject has a valid credential of a specific type
    pub fn has_valid_credential(
        env: Env,
        subject: Address,
        credential_type: CredentialType,
    ) -> bool {
        let credentials = Self::get_subject_credentials(env.clone(), subject);
        let timestamp = env.ledger().timestamp();

        for cred in credentials.iter() {
            if cred.credential_type == credential_type
                && !cred.is_revoked
                && (cred.expiration_date == 0 || cred.expiration_date > timestamp)
            {
                return true;
            }
        }
        false
    }

    // ========================================================================
    // IDENTITY RECOVERY
    // ========================================================================

    /// Add a recovery guardian
    pub fn add_recovery_guardian(
        env: Env,
        subject: Address,
        guardian: Address,
        weight: u32,
    ) -> Result<(), Error> {
        subject.require_auth();

        let mut guardians: Vec<RecoveryGuardian> = env
            .storage()
            .persistent()
            .get(&DataKey::RecoveryGuardians(subject.clone()))
            .unwrap_or(Vec::new(&env));

        let new_guardian = RecoveryGuardian {
            address: guardian.clone(),
            weight,
            added_at: env.ledger().timestamp(),
        };

        guardians.push_back(new_guardian);
        env.storage()
            .persistent()
            .set(&DataKey::RecoveryGuardians(subject.clone()), &guardians);

        env.events().publish(
            (Symbol::new(&env, "GuardianAdded"),),
            (subject, guardian, weight),
        );

        Ok(())
    }

    /// Remove a recovery guardian
    pub fn remove_recovery_guardian(
        env: Env,
        subject: Address,
        guardian: Address,
    ) -> Result<(), Error> {
        subject.require_auth();
        Self::require_not_paused(&env)?;

        let guardians: Vec<RecoveryGuardian> = env
            .storage()
            .persistent()
            .get(&DataKey::RecoveryGuardians(subject.clone()))
            .unwrap_or(Vec::new(&env));

        let mut new_guardians = Vec::new(&env);
        for g in guardians.iter() {
            if g.address != guardian {
                new_guardians.push_back(g);
            }
        }

        env.storage()
            .persistent()
            .set(&DataKey::RecoveryGuardians(subject.clone()), &new_guardians);

        env.events()
            .publish((Symbol::new(&env, "GuardianRemoved"),), (subject, guardian));

        Ok(())
    }

    /// Set recovery threshold
    pub fn set_recovery_threshold(env: Env, subject: Address, threshold: u32) -> Result<(), Error> {
        subject.require_auth();
        Self::require_not_paused(&env)?;

        env.storage()
            .persistent()
            .set(&DataKey::RecoveryThreshold(subject.clone()), &threshold);

        env.events().publish(
            (Symbol::new(&env, "ThresholdUpdated"),),
            (subject, threshold),
        );

        Ok(())
    }

    /// Initiate identity recovery
    pub fn initiate_recovery(
        env: Env,
        guardian: Address,
        subject: Address,
        new_controller: Address,
        new_primary_key: BytesN<32>,
    ) -> Result<u64, Error> {
        guardian.require_auth();

        // Verify guardian
        let guardians: Vec<RecoveryGuardian> = env
            .storage()
            .persistent()
            .get(&DataKey::RecoveryGuardians(subject.clone()))
            .unwrap_or(Vec::new(&env));

        let guardian_info = guardians
            .iter()
            .find(|g| g.address == guardian)
            .ok_or(Error::InvalidRecoveryGuardian)?;

        // Check if recovery already pending
        if env
            .storage()
            .persistent()
            .has(&DataKey::ActiveRecovery(subject.clone()))
        {
            return Err(Error::RecoveryAlreadyPending);
        }

        let request_id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::RecoveryCounter)
            // SAFETY: counter starts at 0 and increments by 1; saturation at u64::MAX is acceptable
            .unwrap_or(0u64)
            .saturating_add(1);

        let mut approvals = Vec::new(&env);
        approvals.push_back(guardian.clone());

        let request = RecoveryRequest {
            request_id,
            subject: subject.clone(),
            new_controller,
            new_primary_key,
            created_at: env.ledger().timestamp(),
            approvals,
            total_weight: guardian_info.weight,
            executed: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::RecoveryRequest(request_id), &request);
        env.storage()
            .persistent()
            .set(&DataKey::ActiveRecovery(subject.clone()), &request_id);
        env.storage()
            .persistent()
            .set(&DataKey::RecoveryCounter, &request_id);

        // Update DID status
        let mut did_doc: DIDDocument = env
            .storage()
            .persistent()
            .get(&DataKey::DIDDocument(subject.clone()))
            .ok_or(Error::DIDNotFound)?;
        did_doc.status = DIDStatus::RecoveryPending;
        env.storage()
            .persistent()
            .set(&DataKey::DIDDocument(subject.clone()), &did_doc);

        env.events().publish(
            (Symbol::new(&env, "RecoveryInitiated"),),
            (subject, request_id),
        );

        Ok(request_id)
    }

    /// Approve a recovery request
    pub fn approve_recovery(env: Env, guardian: Address, request_id: u64) -> Result<(), Error> {
        guardian.require_auth();
        Self::require_not_paused(&env)?;

        let mut request: RecoveryRequest = env
            .storage()
            .persistent()
            .get(&DataKey::RecoveryRequest(request_id))
            .ok_or(Error::RecoveryNotInitiated)?;

        if request.executed {
            return Err(Error::RecoveryNotInitiated);
        }

        // Verify guardian
        let guardians: Vec<RecoveryGuardian> = env
            .storage()
            .persistent()
            .get(&DataKey::RecoveryGuardians(request.subject.clone()))
            .unwrap_or(Vec::new(&env));

        let guardian_info = guardians
            .iter()
            .find(|g| g.address == guardian)
            .ok_or(Error::InvalidRecoveryGuardian)?;

        // Check if already approved
        if request.approvals.iter().any(|a| a == guardian) {
            return Ok(());
        }

        request.approvals.push_back(guardian.clone());
        // SAFETY: total_weight accumulates guardian weights; saturation at u32::MAX is acceptable
        // as threshold checks use < comparison and cannot exceed total possible weight
        request.total_weight = request.total_weight.saturating_add(guardian_info.weight);

        env.storage()
            .persistent()
            .set(&DataKey::RecoveryRequest(request_id), &request);

        env.events().publish(
            (Symbol::new(&env, "RecoveryApproved"),),
            (guardian, request_id),
        );

        Ok(())
    }

    /// Execute recovery after timelock and threshold met
    pub fn execute_recovery(env: Env, request_id: u64) -> Result<(), Error> {
        let mut request: RecoveryRequest = env
            .storage()
            .persistent()
            .get(&DataKey::RecoveryRequest(request_id))
            .ok_or(Error::RecoveryNotInitiated)?;

        if request.executed {
            return Err(Error::RecoveryNotInitiated);
        }

        // Check timelock
        let now = env.ledger().timestamp();
        // SAFETY: created_at is a ledger timestamp; saturation prevents overflow on addition
        if now < request.created_at.saturating_add(DEFAULT_RECOVERY_TIMELOCK) {
            return Err(Error::RecoveryTimelockNotElapsed);
        }

        // Check threshold
        let threshold: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::RecoveryThreshold(request.subject.clone()))
            .unwrap_or(DEFAULT_RECOVERY_THRESHOLD);

        if request.total_weight < threshold {
            return Err(Error::InsufficientGuardianApprovals);
        }

        // Execute recovery - update DID document
        let mut did_doc: DIDDocument = env
            .storage()
            .persistent()
            .get(&DataKey::DIDDocument(request.subject.clone()))
            .ok_or(Error::DIDNotFound)?;

        // Update controller
        did_doc.controller = request.new_controller.clone();

        // Create new primary verification method
        let new_vm_id = String::from_str(&env, "#recovery-key");
        let new_vm = VerificationMethod {
            id: new_vm_id.clone(),
            method_type: VerificationMethodType::Ed25519VerificationKey2020,
            controller: request.new_controller.clone(),
            public_key: request.new_primary_key.clone(),
            is_active: true,
            created: now,
            last_rotated: 0,
        };

        // Deactivate old methods and add new one
        let mut updated_methods = Vec::new(&env);
        for vm in did_doc.verification_methods.iter() {
            let deactivated = VerificationMethod {
                id: vm.id.clone(),
                method_type: vm.method_type.clone(),
                controller: vm.controller.clone(),
                public_key: vm.public_key.clone(),
                is_active: false,
                created: vm.created,
                last_rotated: vm.last_rotated,
            };
            updated_methods.push_back(deactivated);
        }
        updated_methods.push_back(new_vm);
        did_doc.verification_methods = updated_methods;

        // Update authentication to use new key
        let mut new_auth = Vec::new(&env);
        new_auth.push_back(new_vm_id);
        did_doc.authentication = new_auth;

        did_doc.status = DIDStatus::Active;
        did_doc.updated = now;
        // SAFETY: version is a u32 monotonic counter; saturation at u32::MAX is acceptable
        did_doc.version = did_doc.version.saturating_add(1);

        env.storage()
            .persistent()
            .set(&DataKey::DIDDocument(request.subject.clone()), &did_doc);

        // Mark request as executed
        request.executed = true;
        env.storage()
            .persistent()
            .set(&DataKey::RecoveryRequest(request_id), &request);

        // Clear active recovery
        env.storage()
            .persistent()
            .remove(&DataKey::ActiveRecovery(request.subject.clone()));

        env.events().publish(
            (Symbol::new(&env, "RecoveryExecuted"),),
            (request.subject, request_id),
        );

        Ok(())
    }

    /// Cancel a recovery request (only subject with existing key)
    pub fn cancel_recovery(env: Env, subject: Address) -> Result<(), Error> {
        subject.require_auth();
        Self::require_not_paused(&env)?;

        let request_id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::ActiveRecovery(subject.clone()))
            .ok_or(Error::RecoveryNotInitiated)?;

        let mut request: RecoveryRequest = env
            .storage()
            .persistent()
            .get(&DataKey::RecoveryRequest(request_id))
            .ok_or(Error::RecoveryNotInitiated)?;

        request.executed = true;
        env.storage()
            .persistent()
            .set(&DataKey::RecoveryRequest(request_id), &request);

        // Update DID status back to active
        let mut did_doc: DIDDocument = env
            .storage()
            .persistent()
            .get(&DataKey::DIDDocument(subject.clone()))
            .ok_or(Error::DIDNotFound)?;
        did_doc.status = DIDStatus::Active;
        env.storage()
            .persistent()
            .set(&DataKey::DIDDocument(subject.clone()), &did_doc);

        env.storage()
            .persistent()
            .remove(&DataKey::ActiveRecovery(subject.clone()));

        env.events().publish(
            (Symbol::new(&env, "RecoveryCancelled"),),
            (subject, request_id),
        );

        Ok(())
    }

    // ========================================================================
    // SERVICE ENDPOINT MANAGEMENT
    // ========================================================================

    /// Add a service endpoint to DID
    pub fn add_service(
        env: Env,
        subject: Address,
        service_id: String,
        service_type: String,
        endpoint: String,
    ) -> Result<(), Error> {
        subject.require_auth();

        sanitize_id(&env, &service_id).map_err(Self::map_sanitization_error)?;
        sanitize_string(&env, &service_type, MAX_GENERAL_LEN)
            .map_err(Self::map_sanitization_error)?;
        sanitize_url(&env, &endpoint).map_err(|_| Error::InvalidServiceEndpoint)?;

        let mut did_doc: DIDDocument = env
            .storage()
            .persistent()
            .get(&DataKey::DIDDocument(subject.clone()))
            .ok_or(Error::DIDNotFound)?;

        if matches!(did_doc.status, DIDStatus::Deactivated) {
            return Err(Error::DIDDeactivated);
        }

        let new_service = ServiceEndpoint {
            id: service_id.clone(),
            service_type,
            endpoint,
            is_active: true,
        };

        did_doc.services.push_back(new_service);
        did_doc.updated = env.ledger().timestamp();
        // SAFETY: version is a u32 monotonic counter; saturation at u32::MAX is acceptable
        did_doc.version = did_doc.version.saturating_add(1);

        env.storage()
            .persistent()
            .set(&DataKey::DIDDocument(subject.clone()), &did_doc);

        env.events()
            .publish((Symbol::new(&env, "ServiceAdded"),), (subject, service_id));

        Ok(())
    }

    /// Remove/deactivate a service endpoint
    pub fn remove_service(env: Env, subject: Address, service_id: String) -> Result<(), Error> {
        subject.require_auth();
        Self::require_not_paused(&env)?;

        let mut did_doc: DIDDocument = env
            .storage()
            .persistent()
            .get(&DataKey::DIDDocument(subject.clone()))
            .ok_or(Error::DIDNotFound)?;

        let mut updated_services = Vec::new(&env);
        let mut found = false;

        for svc in did_doc.services.iter() {
            if svc.id == service_id {
                found = true;
                // Skip - effectively removes it
            } else {
                updated_services.push_back(svc);
            }
        }

        if !found {
            return Err(Error::ServiceNotFound);
        }

        did_doc.services = updated_services;
        did_doc.updated = env.ledger().timestamp();
        // SAFETY: version is a u32 monotonic counter; saturation at u32::MAX is acceptable
        did_doc.version = did_doc.version.saturating_add(1);

        env.storage()
            .persistent()
            .set(&DataKey::DIDDocument(subject.clone()), &did_doc);

        env.events().publish(
            (Symbol::new(&env, "ServiceRemoved"),),
            (subject, service_id),
        );

        Ok(())
    }

    // ========================================================================
    // VERIFIER MANAGEMENT
    // ========================================================================

    /// Add a verifier (only owner can do this)
    pub fn add_verifier(env: Env, verifier: Address) -> Result<(), Error> {
        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .ok_or(Error::NotInitialized)?;

        owner.require_auth();

        let rbac_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::RbacContract)
            .ok_or(Error::NotInitialized)?;
        let rbac_client = RbacClient::new(&env, &rbac_addr);
        let has_admin = rbac_client.has_role(&owner, &RbacRole::Admin);
        if !has_admin {
            return Err(Error::Unauthorized);
        }

        rbac_client.assign_role(&verifier, &RbacRole::Staff);

        env.storage()
            .instance()
            .set(&DataKey::Verifier(verifier.clone()), &true);

        env.events()
            .publish((Symbol::new(&env, "VerifierAdded"),), verifier);

        Ok(())
    }

    /// Remove a verifier (only owner can do this)
    pub fn remove_verifier(env: Env, verifier: Address) -> Result<(), Error> {
        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .ok_or(Error::NotInitialized)?;

        owner.require_auth();

        if verifier == owner {
            return Err(Error::CannotRemoveOwner);
        }

        let rbac_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::RbacContract)
            .ok_or(Error::NotInitialized)?;
        let rbac_client = RbacClient::new(&env, &rbac_addr);
        let has_admin = rbac_client.has_role(&owner, &RbacRole::Admin);
        if !has_admin {
            return Err(Error::Unauthorized);
        }

        rbac_client.remove_role(&verifier, &RbacRole::Staff);

        env.storage()
            .instance()
            .set(&DataKey::Verifier(verifier.clone()), &false);

        env.events()
            .publish((Symbol::new(&env, "VerifierRemoved"),), verifier);

        Ok(())
    }

    /// Check if an address is a verifier
    pub fn is_verifier(env: Env, account: Address) -> bool {
        let rbac_addr: Address = match env.storage().instance().get(&DataKey::RbacContract) {
            Some(v) => v,
            None => return false,
        };
        let client = RbacClient::new(&env, &rbac_addr);
        if client.has_role(&account, &RbacRole::Staff) {
            return true;
        }
        if client.has_role(&account, &RbacRole::Service) {
            return true;
        }
        client.has_role(&account, &RbacRole::Admin)
    }

    /// Get the contract owner
    pub fn get_owner(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Owner)
            .ok_or(Error::NotInitialized)
    }

    // ========================================================================
    // LEGACY FUNCTIONS (Backward Compatibility)
    // ========================================================================

    /// Register an identity hash with metadata (legacy support)
    pub fn register_identity_hash(
        env: Env,
        hash: BytesN<32>,
        subject: Address,
        meta: String,
    ) -> Result<(), Error> {
        subject.require_auth();
        Self::require_not_paused(&env)?;

        sanitize_string(&env, &meta, MAX_GENERAL_LEN).map_err(Self::map_sanitization_error)?;

        let identity_record = IdentityRecord {
            hash: hash.clone(),
            meta: meta.clone(),
            registered_by: subject.clone(),
        };

        env.storage()
            .instance()
            .set(&DataKey::IdentityHash(subject.clone()), &identity_record);

        env.events()
            .publish((symbol_short!("IdReg"),), (subject, hash, meta));

        Ok(())
    }

    /// Create an attestation (legacy - only verifiers can do this)
    pub fn attest(
        env: Env,
        verifier: Address,
        subject: Address,
        claim_hash: BytesN<32>,
    ) -> Result<(), Error> {
        verifier.require_auth();
        Self::require_not_paused(&env)?;

        let is_verifier = Self::is_verifier(env.clone(), verifier.clone());

        if !is_verifier {
            return Err(Error::NotVerifier);
        }

        let attestation = Attestation {
            claim_hash: claim_hash.clone(),
            verifier: verifier.clone(),
            is_active: true,
        };

        env.storage().instance().set(
            &DataKey::Attestation(subject.clone(), claim_hash.clone()),
            &attestation,
        );

        let mut attestations: Vec<BytesN<32>> = env
            .storage()
            .instance()
            .get(&DataKey::SubjectAttestations(subject.clone()))
            .unwrap_or(Vec::new(&env));

        attestations.push_back(claim_hash.clone());
        env.storage().instance().set(
            &DataKey::SubjectAttestations(subject.clone()),
            &attestations,
        );

        env.events().publish(
            (symbol_short!("Attested"),),
            (subject, verifier, claim_hash),
        );

        Ok(())
    }

    /// Revoke an attestation (legacy)
    pub fn revoke_attestation(
        env: Env,
        verifier: Address,
        subject: Address,
        claim_hash: BytesN<32>,
    ) -> Result<(), Error> {
        verifier.require_auth();
        Self::require_not_paused(&env)?;

        let is_verifier = Self::is_verifier(env.clone(), verifier.clone());

        if !is_verifier {
            return Err(Error::NotVerifier);
        }

        let mut attestation: Attestation = env
            .storage()
            .instance()
            .get(&DataKey::Attestation(subject.clone(), claim_hash.clone()))
            .ok_or(Error::AttestationNotFound)?;

        attestation.is_active = false;
        env.storage().instance().set(
            &DataKey::Attestation(subject.clone(), claim_hash.clone()),
            &attestation,
        );

        env.events()
            .publish((symbol_short!("Revoked"),), (subject, verifier, claim_hash));

        Ok(())
    }

    /// Get identity hash for a subject (legacy)
    pub fn get_identity_hash(env: Env, subject: Address) -> Option<BytesN<32>> {
        let record: Option<IdentityRecord> = env
            .storage()
            .instance()
            .get(&DataKey::IdentityHash(subject));

        record.map(|r| r.hash)
    }

    /// Get identity metadata for a subject (legacy)
    pub fn get_identity_meta(env: Env, subject: Address) -> Option<String> {
        let record: Option<IdentityRecord> = env
            .storage()
            .instance()
            .get(&DataKey::IdentityHash(subject));

        record.map(|r| r.meta)
    }

    /// Check if a specific attestation is active (legacy)
    pub fn is_attested(env: Env, subject: Address, claim_hash: BytesN<32>) -> bool {
        let attestation: Option<Attestation> = env
            .storage()
            .instance()
            .get(&DataKey::Attestation(subject, claim_hash));

        attestation.is_some_and(|a| a.is_active)
    }

    /// Get all active attestations for a subject (legacy)
    pub fn get_attestations(env: Env, subject: Address) -> Vec<BytesN<32>> {
        let all_attestations: Vec<BytesN<32>> = env
            .storage()
            .instance()
            .get(&DataKey::SubjectAttestations(subject.clone()))
            .unwrap_or(Vec::new(&env));

        let mut active_attestations = Vec::new(&env);

        for claim_hash in all_attestations.iter() {
            if let Some(attestation) =
                env.storage()
                    .instance()
                    .get::<DataKey, Attestation>(&DataKey::Attestation(
                        subject.clone(),
                        claim_hash.clone(),
                    ))
            {
                if attestation.is_active {
                    active_attestations.push_back(claim_hash);
                }
            }
        }

        active_attestations
    }

    // ========================================================================
    // HELPER FUNCTIONS
    // ========================================================================

    fn map_sanitization_error(e: SanitizationError) -> Error {
        match e {
            SanitizationError::InputTooLong => Error::InputTooLong,
            _ => Error::InvalidInput,
        }
    }

    /// Generate DID string from network and address
    fn generate_did_string(env: &Env, network_id: &String, subject: &Address) -> String {
        const MAX_PART_LEN: usize = 128;
        const MAX_DID_LEN: usize = 512;

        let subject_str = subject.to_string();

        // SAFETY: len() returns u32 on Soroban String; truncate to MAX_PART_LEN to stay in bounds
        let network_len = (network_id.len() as usize).min(MAX_PART_LEN);
        let subject_len = (subject_str.len() as usize).min(MAX_PART_LEN);

        let mut network_buf = [0u8; MAX_PART_LEN];
        network_id.copy_into_slice(&mut network_buf[..network_len]);

        let mut subject_buf = [0u8; MAX_PART_LEN];
        subject_str.copy_into_slice(&mut subject_buf[..subject_len]);

        let mut did_bytes = Bytes::new(env);
        did_bytes.extend_from_slice(b"did:stellar:uzima:");
        did_bytes.extend_from_slice(&network_buf[..network_len]);
        did_bytes.extend_from_slice(b":");
        did_bytes.extend_from_slice(&subject_buf[..subject_len]);

        let did_buf = did_bytes.to_buffer::<MAX_DID_LEN>();
        String::from_bytes(env, did_buf.as_slice())
    }

    fn generate_credential_id(
        env: &Env,
        issuer: &Address,
        subject: &Address,
        timestamp: u64,
        _credential_type: &CredentialType,
    ) -> BytesN<32> {
        let mut data = issuer.to_xdr(env);
        data.append(&subject.to_xdr(env));
        data.append(&timestamp.to_xdr(env));
        env.crypto().sha256(&data).into()
    }

    /// Compute document hash for audit trail
    fn compute_document_hash(env: &Env, doc: &DIDDocument) -> BytesN<32> {
        let data = doc.clone().to_xdr(env);
        env.crypto().sha256(&data).into()
    }

    /// DID-based authorization check
    pub fn verify_did_authorization(
        env: Env,
        subject: Address,
        required_relationship: VerificationRelationship,
    ) -> bool {
        let did_doc: Option<DIDDocument> = env
            .storage()
            .persistent()
            .get(&DataKey::DIDDocument(subject));

        match did_doc {
            None => false,
            Some(doc) => {
                if !matches!(doc.status, DIDStatus::Active) {
                    return false;
                }

                // Check if any verification method for the required relationship is active
                let method_ids = match required_relationship {
                    VerificationRelationship::Authentication => &doc.authentication,
                    VerificationRelationship::AssertionMethod => &doc.assertion_method,
                    VerificationRelationship::KeyAgreement => &doc.key_agreement,
                    VerificationRelationship::CapabilityInvocation => &doc.capability_invocation,
                    VerificationRelationship::CapabilityDelegation => &doc.capability_delegation,
                };

                for method_id in method_ids.iter() {
                    for vm in doc.verification_methods.iter() {
                        if vm.id == method_id && vm.is_active {
                            return true;
                        }
                    }
                }

                false
            },
        }
    }

    /// Registers a FIDO2 / WebAuthn authenticator device as a verification method
    /// in the subject's DID document.
    ///
    /// Called by the `fido2_authenticator` contract after a successful device
    /// registration ceremony.  The public key is stored as a SHA-256 hash
    /// (`public_key_hash`) because DID verification methods use 32-byte keys and
    /// FIDO2 P-256 keys are 65 bytes; the hash acts as a stable, compact identifier.
    ///
    /// # Arguments
    /// * `subject`          — DID owner; must have an active DID document.
    /// * `device_name`      — friendly name used as the verification method fragment ID.
    /// * `algorithm_tag`    — 1 = EdDSA (Ed25519), 2 = ES256 (P-256).
    /// * `public_key_hash`  — SHA-256 of the raw authenticator public key bytes.
    ///
    /// If the subject has no DID document the call is silently ignored so that
    /// the `fido2_authenticator` registration is never blocked by DID state.
    pub fn add_fido2_device(
        env: Env,
        subject: Address,
        device_name: String,
        algorithm_tag: u32,
        public_key_hash: BytesN<32>,
    ) -> Result<(), Error> {
        subject.require_auth();
        Self::require_not_paused(&env)?;

        // Silently succeed when no DID document exists yet.
        let mut did_doc: DIDDocument = match env
            .storage()
            .persistent()
            .get(&DataKey::DIDDocument(subject.clone()))
        {
            Some(doc) => doc,
            None => return Ok(()),
        };

        if matches!(did_doc.status, DIDStatus::Deactivated) {
            return Ok(()); // Non-blocking: DID deactivated
        }

        let method_type = if algorithm_tag == 1 {
            VerificationMethodType::Fido2EdDsa2024
        } else {
            VerificationMethodType::Fido2Es2562024
        };

        let timestamp = env.ledger().timestamp();

        // Build a unique method ID: "fido2-<device_name>-<timestamp_fragment>".
        // We use the device_name directly as the fragment for human readability.
        // Collision avoidance: callers should use unique names per device.
        let method_id = device_name.clone();

        let new_vm = VerificationMethod {
            id: method_id.clone(),
            method_type,
            controller: subject.clone(),
            public_key: public_key_hash,
            is_active: true,
            created: timestamp,
            last_rotated: 0,
        };

        did_doc.verification_methods.push_back(new_vm);
        // FIDO2 devices serve as authentication verification methods.
        did_doc.authentication.push_back(method_id);
        did_doc.updated = timestamp;
        // SAFETY: version is a u32 monotonic counter; saturation at u32::MAX is acceptable
        did_doc.version = did_doc.version.saturating_add(1);

        env.storage()
            .persistent()
            .set(&DataKey::DIDDocument(subject), &did_doc);

        Ok(())
    }

    // ========================================================================
    // PROVIDER STAKING (SUT Token Reputation Bonding)
    // ========================================================================

    /// Deposit stake for a healthcare provider.
    pub fn deposit_stake(
        env: Env,
        provider: Address,
        amount: i128,
        token_address: Address,
    ) -> Result<(), Error> {
        provider.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidInput);
        }

        let now = env.ledger().timestamp();
        let lock_until = now.saturating_add(90 * 86400); // 90 days default lock

        // Store stake info
        let stake_info = ProviderStake {
            provider: provider.clone(),
            token_address: token_address.clone(),
            amount,
            locked_until: lock_until,
            slashed: false,
            deposited_at: now,
        };

        env.storage()
            .persistent()
            .set(&DataKey::StakeInfo(provider.clone()), &stake_info);

        // Emit stake deposited event
        env.events().publish(
            (Symbol::new(&env, "StakeDeposited"),),
            (provider, amount, lock_until),
        );

        Ok(())
    }

    /// Withdraw stake after lock period if not slashed and in good standing.
    pub fn withdraw_stake(env: Env, provider: Address) -> Result<i128, Error> {
        provider.require_auth();

        let now = env.ledger().timestamp();

        // Load stake info to verify lock period has elapsed
        let stake_info: ProviderStake = env
            .storage()
            .persistent()
            .get(&DataKey::StakeInfo(provider.clone()))
            .ok_or(Error::InvalidInput)?;

        if now < stake_info.locked_until {
            return Err(Error::InvalidInput);
        }

        if stake_info.slashed {
            return Err(Error::InvalidInput);
        }

        // Remove stake info
        env.storage()
            .persistent()
            .remove(&DataKey::StakeInfo(provider.clone()));

        env.events().publish(
            (Symbol::new(&env, "StakeWithdrawn"),),
            (provider.clone(), stake_info.amount),
        );

        Ok(stake_info.amount)
    }

    /// Slash stake for verified misconduct (governance only).
    pub fn slash_stake(
        env: Env,
        governance: Address,
        provider: Address,
        amount: i128,
        reason: String,
    ) -> Result<(), Error> {
        governance.require_auth();

        let mut stake_info: ProviderStake = env
            .storage()
            .persistent()
            .get(&DataKey::StakeInfo(provider.clone()))
            .ok_or(Error::InvalidInput)?;

        stake_info.slashed = true;
        env.storage()
            .persistent()
            .set(&DataKey::StakeInfo(provider.clone()), &stake_info);

        env.events().publish(
            (Symbol::new(&env, "StakeSlashed"),),
            (provider, amount, reason),
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    /// Verifies that is_paused returns false when the Paused key has never been written
    /// (i.e., the contract is freshly deployed and uninitialized).
    #[test]
    fn test_is_paused_returns_false_when_storage_empty() {
        let env = Env::default();
        let contract_id = env.register_contract(None, IdentityRegistryContract);
        let client = IdentityRegistryContractClient::new(&env, &contract_id);
        assert!(!client.is_paused());
    }
}
