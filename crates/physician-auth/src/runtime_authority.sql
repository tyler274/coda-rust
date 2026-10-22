SELECT
    s.expires_at
FROM
    physician_sessions AS s
    INNER JOIN physician_accounts AS a ON a.id = s.account_id
    INNER JOIN physician_memberships AS m ON m.practice_id = a.practice_id
        AND m.physician_id = a.physician_id
    INNER JOIN physician_patients AS p ON p.patient_workspace_id = m.patient_workspace_id
WHERE
    s.token_hash = $1
    AND a.practice_id = $2
    AND a.physician_id = $3
    AND m.patient_workspace_id = $4
    AND p.practice_id = a.practice_id
    AND a.active
    AND s.created_at >= a.sessions_ended_before
    AND m.allowed FOR SHARE OF a,
    s,
    m,
    p;
