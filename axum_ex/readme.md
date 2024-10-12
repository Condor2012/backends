# Axum + sqlx example
To run mysql container:
```bash
podman run --name mysql_example -e MYSQL_ROOT_PASSWORD=pass -e MYSQL_DATABASE=boards_test -p 3306:3306 mysql:latest
```

To create db:
```bash
podman exec -it mysql_example /bin/bash 

mysql -uroot -p

use boards_test;
```

```sql
CREATE TABLE boards
(
	id INT PRIMARY KEY AUTO_INCREMENT,
	name VARCHAR(255) NOT NULL
);

CREATE TABLE tasks
(
	board_id INT,
	name VARCHAR(255) NOT NULL,
	description VARCHAR(255) NOT NULL,
	FOREIGN KEY (board_id)  REFERENCES boards (id) ON DELETE CASCADE
);
```