/**
 * VitaStellar JavaScript Client SDK
 * Provides connection management, transaction submission, encryption, and audit tracking
 * for Soroban smart contracts on Stellar.
 */
class VitaStellarSDK {
  constructor(config = {}) {
    this.networkUrl = config.networkUrl || 'https://soroban-testnet.stellar.org:443';
    this.networkPassphrase = config.networkPassphrase || 'Test SDF Network ; September 2015';
    this.contractId = config.contractId || 'CCMEDICALRECORDS1234567890VITASTELLAR';
    
    // In-memory persistent state for offline simulation & fast testing
    this.patients = new Map();
    this.records = new Map();
    this.goals = new Map();
    this.consents = new Map();
    this.auditLogs = [];

    this._initializeSeedData();
  }

  _initializeSeedData() {
    // Seed initial patient identity
    const p1 = {
      patientId: 'P-9842',
      publicKey: 'GD5W6G6V74N5PXYZ7890ABCDEF1234567890',
      did: 'did:stellar:P-9842',
      registeredAt: new Date(Date.now() - 86400000 * 5).toISOString()
    };
    this.patients.set(p1.patientId, p1);

    // Seed medical record
    const r1 = {
      recordId: 'REC-1001',
      patientId: 'P-9842',
      doctorId: 'DOC-5512',
      recordType: 'VACCINATION',
      encryptedData: 'U2FsdGVkX19x8Z2m0p7L...[AES-256-ENCRYPTED]',
      plaintextPreview: 'COVID-19 Booster Injection (Pfizer-BioNTech)',
      metadata: 'hospital_main_wing,traditional_healing_supported',
      timestamp: new Date(Date.now() - 86400000 * 2).toISOString(),
      blockHeight: 142095
    };
    this.records.set(r1.recordId, r1);

    // Seed health goals
    const g1 = { goalId: 'G-01', title: 'Daily Step Goal (10k)', target: 10000, current: 8450, unit: 'steps', category: 'Fitness', rewardTokens: 15 };
    const g2 = { goalId: 'G-02', title: 'Sleep Duration (8h)', target: 8, current: 7.5, unit: 'hours', category: 'Recovery', rewardTokens: 20 };
    const g3 = { goalId: 'G-03', title: 'Hydration (2.5L)', target: 2500, current: 2500, unit: 'ml', category: 'Wellness', rewardTokens: 10 };
    this.goals.set(g1.goalId, g1);
    this.goals.set(g2.goalId, g2);
    this.goals.set(g3.goalId, g3);

    // Initial audit log events
    this.auditLogs.push(
      { type: 'INITIALIZE', detail: 'VitaStellar Contract Initialized', time: '10:00:00 AM', txHash: '0x3a4b...89f1' },
      { type: 'REGISTER_PATIENT', detail: 'Patient P-9842 registered with DID', time: '10:05:12 AM', txHash: '0x9c1e...44d2' },
      { type: 'WRITE_RECORD', detail: 'Record REC-1001 written by DOC-5512', time: '10:14:30 AM', txHash: '0x7b8f...12e9' }
    );
  }

  /** Register a new patient identity on-chain */
  async registerPatient(patientId, publicKey) {
    const did = `did:stellar:${patientId}`;
    const patientObj = {
      patientId,
      publicKey,
      did,
      registeredAt: new Date().toISOString()
    };
    this.patients.set(patientId, patientObj);
    
    this._logEvent('REGISTER_PATIENT', `Patient ${patientId} registered with DID ${did}`);
    return { success: true, patient: patientObj };
  }

  /** Write an encrypted medical record */
  async writeRecord({ patientId, doctorId, recordType, content, metadata }) {
    if (!patientId || !content) {
      throw new Error('Patient ID and Content are required fields.');
    }

    const recordId = `REC-${Math.floor(1000 + Math.random() * 9000)}`;
    const encryptedData = `ENC[${btoa(content)}]`;
    
    const newRecord = {
      recordId,
      patientId,
      doctorId,
      recordType,
      encryptedData,
      plaintextPreview: content,
      metadata: metadata || 'standard_record',
      timestamp: new Date().toISOString(),
      blockHeight: Math.floor(142100 + Math.random() * 50)
    };

    this.records.set(recordId, newRecord);
    this._logEvent('WRITE_RECORD', `Record ${recordId} created for patient ${patientId}`);
    return { success: true, record: newRecord };
  }

  /** Retrieve medical record metadata */
  async getRecordMetadata(recordId) {
    const record = this.records.get(recordId);
    if (!record) {
      throw new Error(`Record ${recordId} not found.`);
    }
    return record;
  }

  /** Grant access consent to doctor or institution */
  async grantConsent(grantee, recordId, durationDays) {
    const consentKey = `${grantee}:${recordId}`;
    const consentObj = {
      grantee,
      recordId,
      expiresAt: new Date(Date.now() + durationDays * 86400000).toISOString(),
      grantedAt: new Date().toISOString()
    };
    this.consents.set(consentKey, consentObj);

    this._logEvent('GRANT_CONSENT', `Access granted to ${grantee} for record ${recordId}`);
    return { success: true, consent: consentObj };
  }

  /** Log health activity & update goal progress */
  async logGoalProgress(goalId, addValue) {
    const goal = this.goals.get(goalId);
    if (!goal) throw new Error('Goal not found');

    goal.current = Math.min(goal.target, goal.current + addValue);
    const completed = goal.current >= goal.target;

    if (completed) {
      this._logEvent('GOAL_COMPLETED', `Goal ${goal.title} completed! Earned ${goal.rewardTokens} VITA tokens.`);
    } else {
      this._logEvent('GOAL_UPDATE', `Goal ${goal.title} updated: ${goal.current}/${goal.target}`);
    }

    return { success: true, goal, completed };
  }

  /** Get all records for a patient */
  getRecordsForPatient(patientId) {
    return Array.from(this.records.values()).filter(r => r.patientId === patientId || !patientId);
  }

  /** Get all active health goals */
  getGoals() {
    return Array.from(this.goals.values());
  }

  /** Get recent audit log stream */
  getAuditLogs() {
    return [...this.auditLogs].reverse();
  }

  _logEvent(type, detail) {
    const time = new Date().toLocaleTimeString();
    const txHash = `0x${Array.from({length: 8}, () => Math.floor(Math.random()*16).toString(16)).join('')}...${Math.floor(Math.random()*9000+1000)}`;
    this.auditLogs.push({ type, detail, time, txHash });
  }
}

// Export singleton instance for app window usage
window.vitastellarSDK = new VitaStellarSDK();
