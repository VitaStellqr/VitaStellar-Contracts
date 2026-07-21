/**
 * VitaStellar Web DApp Main Application Controller
 */
document.addEventListener('DOMContentLoaded', () => {
  const sdk = window.vitastellarSDK;

  // Navigation setup
  const navItems = document.querySelectorAll('.nav-item');
  const tabViews = document.querySelectorAll('.tab-view');
  const pageTitle = document.getElementById('current-tab-title');

  const tabTitles = {
    'dashboard': 'Dashboard Overview',
    'medical-records': 'Medical Records Management',
    'wellness-goals': 'Wellness & Health Tracker',
    'consent-management': 'Patient Consent & Access Control',
    'audit-trail': 'On-Chain Audit Log Stream'
  };

  navItems.forEach(item => {
    item.addEventListener('click', () => {
      const tabKey = item.getAttribute('data-tab');
      
      navItems.forEach(nav => nav.classList.remove('active'));
      tabViews.forEach(view => view.classList.remove('active'));

      item.classList.add('active');
      document.getElementById(`tab-${tabKey}`).classList.add('active');
      pageTitle.textContent = tabTitles[tabKey] || 'VitaStellar DApp';
    });
  });

  // Render initial views
  refreshAllViews();

  // Handle write record submission
  const writeRecordForm = document.getElementById('form-write-record');
  writeRecordForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const patientId = document.getElementById('input-patient-id').value;
    const doctorId = document.getElementById('input-doctor-id').value;
    const recordType = document.getElementById('input-record-type').value;
    const metadata = document.getElementById('input-metadata').value;
    const content = document.getElementById('input-content').value;

    try {
      const result = await sdk.writeRecord({ patientId, doctorId, recordType, content, metadata });
      showToast(`Record ${result.record.recordId} successfully encrypted and saved to Soroban contract!`, 'success');
      writeRecordForm.reset();
      document.getElementById('input-patient-id').value = 'P-9842';
      document.getElementById('input-doctor-id').value = 'DOC-7721';
      refreshAllViews();
    } catch (err) {
      showToast(`Error: ${err.message}`, 'danger');
    }
  });

  // Handle grant consent submission
  const consentForm = document.getElementById('form-grant-consent');
  consentForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    const grantee = document.getElementById('input-grantee').value;
    const recordId = document.getElementById('input-consent-record-id').value;
    const duration = parseInt(document.getElementById('input-duration').value, 10);

    try {
      const res = await sdk.grantConsent(grantee, recordId, duration);
      showToast(`Consent granted for ${grantee} to access ${recordId} for ${duration} days!`, 'success');
      consentForm.reset();
      refreshAllViews();
    } catch (err) {
      showToast(`Error granting consent: ${err.message}`, 'danger');
    }
  });

  // Top header CTA button
  document.getElementById('open-new-record-modal').addEventListener('click', () => {
    document.querySelector('[data-tab="medical-records"]').click();
  });

  document.getElementById('btn-refresh-dashboard').addEventListener('click', () => {
    refreshAllViews();
    showToast('State refreshed from Soroban contract instance.', 'info');
  });

  // Functions to render views
  function refreshAllViews() {
    renderMetrics();
    renderDashboardTable();
    renderFullRecordsTable();
    renderGoalsGrid();
    renderAuditLogs();
  }

  function renderMetrics() {
    const records = sdk.getRecordsForPatient('P-9842');
    document.getElementById('metric-records-count').textContent = records.length;
    
    const goals = sdk.getGoals();
    document.getElementById('metric-goals-count').textContent = `${goals.length} Goals`;

    let totalReward = 45;
    goals.forEach(g => {
      if (g.current >= g.target) totalReward += g.rewardTokens;
    });
    document.getElementById('metric-rewards-balance').textContent = `${totalReward} VITA`;
  }

  function renderDashboardTable() {
    const records = sdk.getRecordsForPatient('P-9842');
    const tbody = document.getElementById('dashboard-records-tbody');
    tbody.innerHTML = '';

    records.forEach(r => {
      const tr = document.createElement('tr');
      tr.innerHTML = `
        <td><strong style="color:var(--accent-cyan);">${r.recordId}</strong></td>
        <td><span class="tag tag-cyan">${r.recordType}</span></td>
        <td>${r.patientId}</td>
        <td>${r.doctorId}</td>
        <td><code>AES-256-GCM</code></td>
        <td>${new Date(r.timestamp).toLocaleTimeString()}</td>
      `;
      tbody.appendChild(tr);
    });
  }

  function renderFullRecordsTable() {
    const records = sdk.getRecordsForPatient('P-9842');
    const tbody = document.getElementById('full-records-tbody');
    tbody.innerHTML = '';

    records.forEach(r => {
      const tr = document.createElement('tr');
      tr.innerHTML = `
        <td><strong style="color:var(--accent-cyan);">${r.recordId}</strong></td>
        <td><span class="tag tag-purple">${r.recordType}</span></td>
        <td><em>"${r.plaintextPreview}"</em></td>
        <td><span class="tag tag-emerald">${r.metadata}</span></td>
        <td><code>#${r.blockHeight}</code></td>
        <td><button class="btn-secondary" style="padding:4px 10px; font-size:0.75rem;" onclick="alert('Record ${r.recordId} Payload: ${r.encryptedData}')">View Cipher</button></td>
      `;
      tbody.appendChild(tr);
    });
  }

  function renderGoalsGrid() {
    const goals = sdk.getGoals();
    const container = document.getElementById('goals-cards-container');
    container.innerHTML = '';

    goals.forEach(g => {
      const pct = Math.min(100, Math.round((g.current / g.target) * 100));
      const card = document.createElement('div');
      card.className = 'goal-card';
      card.innerHTML = `
        <div class="goal-header">
          <div class="goal-title">${g.title}</div>
          <span class="tag tag-emerald">+${g.rewardTokens} VITA</span>
        </div>
        <div>
          <div style="display:flex; justify-content:space-between; font-size:0.85rem; margin-bottom:6px;">
            <span style="color:var(--text-secondary);">${g.current} / ${g.target} ${g.unit}</span>
            <strong style="color:var(--accent-cyan);">${pct}%</strong>
          </div>
          <div class="progress-bar-bg">
            <div class="progress-bar-fill" style="width: ${pct}%;"></div>
          </div>
        </div>
        <button class="btn-secondary log-progress-btn" data-goal-id="${g.goalId}">+ Log Progress</button>
      `;
      container.appendChild(card);
    });

    // Attach button handlers for logging progress
    document.querySelectorAll('.log-progress-btn').forEach(btn => {
      btn.addEventListener('click', async (e) => {
        const goalId = e.target.getAttribute('data-goal-id');
        const increment = goalId === 'G-01' ? 1500 : (goalId === 'G-02' ? 0.5 : 250);
        const res = await sdk.logGoalProgress(goalId, increment);
        showToast(`Logged +${increment} to ${res.goal.title}!`, 'success');
        refreshAllViews();
      });
    });
  }

  function renderAuditLogs() {
    const logs = sdk.getAuditLogs();
    const container = document.getElementById('audit-logs-container');
    container.innerHTML = '';

    logs.forEach(log => {
      const item = document.createElement('div');
      item.className = 'audit-item';
      item.innerHTML = `
        <div>
          <strong style="color:var(--accent-cyan);">${log.type}:</strong> ${log.detail}
        </div>
        <div style="display:flex; gap:16px; align-items:center;">
          <code>${log.txHash}</code>
          <span class="audit-time">${log.time}</span>
        </div>
      `;
      container.appendChild(item);
    });
  }

  function showToast(message, type = 'info') {
    const container = document.getElementById('toast-container');
    const toast = document.createElement('div');
    toast.className = 'toast';
    toast.innerHTML = `<span>✨</span> <div>${message}</div>`;
    container.appendChild(toast);

    setTimeout(() => {
      toast.style.opacity = '0';
      setTimeout(() => toast.remove(), 300);
    }, 4000);
  }
});
