SELECT
	*,
	ARRAY(
		SELECT username
		FROM users
		WHERE teams.id = users.team_id
	) AS members,
	ARRAY(
		SELECT (
			SELECT challenge_name FROM challenges
			WHERE teams.id = solves.challenge_id
        	)
		FROM solves
		WHERE teams.id = solves.team_id
	) AS solves
FROM teams;
