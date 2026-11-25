// Sadar Badge — walletless simulation of the Ootle template's state machine.
// Mirrors template/src/lib.rs rules: issue/revoke are facilitator-only
// (admin badge), verify is public, revoked badges stop verifying.
// No network calls, no wallet, no keys. State lives only in this page.
(function () {
  var badges = {}; // id -> {learner, level, programme, revoked}
  var counter = 0;

  function $(id) { return document.getElementById(id); }
  function isFacilitator() { return $('isFacilitator').checked; }

  function render() {
    var rows = $('rows');
    var ids = Object.keys(badges);
    if (!ids.length) {
      rows.innerHTML = '<tr><td colspan="5"><small>No badges yet — issue one above.</small></td></tr>';
      return;
    }
    rows.innerHTML = ids.map(function (id) {
      var b = badges[id];
      var cls = b.revoked ? ' class="revoked"' : '';
      var status = b.revoked ? 'REVOKED' : 'VALID';
      return '<tr' + cls + '><td>' + id + '</td><td>' + esc(b.learner) + '</td><td>' +
        esc(b.level) + '</td><td>' + esc(b.programme) + '</td><td>' + status + '</td></tr>';
    }).join('');
  }

  function esc(s) {
    return String(s).replace(/[&<>"']/g, function (c) {
      return { '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[c];
    });
  }

  $('issueBtn').onclick = function () {
    var msg = $('issueMsg');
    if (!isFacilitator()) { msg.innerHTML = '<span class="bad">Denied: issuing requires the facilitator admin badge.</span>'; return; }
    var learner = $('learner').value.trim(), level = $('level').value.trim(), programme = $('programme').value.trim();
    if (!learner || !level || !programme) { msg.innerHTML = '<span class="bad">All three fields are required.</span>'; return; }
    counter += 1;
    var id = 'badge_' + counter;
    badges[id] = { learner: learner, level: level, programme: programme, revoked: false };
    $('verifyId').value = id; $('revokeId').value = id;
    msg.innerHTML = '<span class="ok">Badge <code>' + id + '</code> issued to ' + esc(learner) + ' (' + esc(level) + ').</span>';
    render();
  };

  $('verifyBtn').onclick = function () {
    var id = $('verifyId').value.trim(), msg = $('verifyMsg');
    var b = badges[id];
    if (!b) { msg.innerHTML = '<span class="bad">Unknown badge: no such badge exists.</span>'; return; }
    if (b.revoked) { msg.innerHTML = '<span class="bad">Badge <code>' + esc(id) + '</code> was REVOKED — no longer valid.</span>'; return; }
    msg.innerHTML = '<span class="ok">VALID — ' + esc(b.learner) + ' completed ' + esc(b.level) +
      ' (' + esc(b.programme) + '). Badge <code>' + esc(id) + '</code>.</span>';
  };

  $('revokeBtn').onclick = function () {
    var id = $('revokeId').value.trim(), msg = $('revokeMsg');
    if (!isFacilitator()) { msg.innerHTML = '<span class="bad">Denied: revoking requires the facilitator admin badge.</span>'; return; }
    var b = badges[id];
    if (!b) { msg.innerHTML = '<span class="bad">Unknown badge: no such badge exists.</span>'; return; }
    if (b.revoked) { msg.innerHTML = '<span class="bad">Badge <code>' + esc(id) + '</code> is already revoked.</span>'; return; }
    b.revoked = true;
    msg.innerHTML = '<span class="ok">Badge <code>' + esc(id) + '</code> revoked.</span>';
    render();
  };

  render();
})();
