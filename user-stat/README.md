# User Stats

## 24-09-09

### create db and run migrate

```sh
> createdb user_stats

user-stat (main) [1]> sqlx migrate run
Applied 20240908230124/migrate init (15.716333ms)
Applied 20240908230502/migrate index (10.114ms)
```

```sh
user_stats=> \dt
               List of relations
 Schema |       Name       | Type  |   Owner   
--------+------------------+-------+-----------
 public | _sqlx_migrations | table | postgres
 public | user_stats       | table | postgres
(2 rows)

user_stats=> \d user_stats
                                   Table "public.user_stats"
          Column          |           Type           | Collation | Nullable |      Default      
--------------------------+--------------------------+-----------+----------+-------------------
 email                    | character varying(128)   |           | not null | 
 name                     | character varying(64)    |           | not null | 
 gender                   | gender                   |           |          | 'unknown'::gender
 created_at               | timestamp with time zone |           |          | CURRENT_TIMESTAMP
 last_visited_at          | timestamp with time zone |           | not null | 
 last_watched_at          | timestamp with time zone |           | not null | 
 recent_watched           | integer[]                |           |          | 
 viewed_but_not_started   | integer[]                |           |          | 
 started_but_not_finished | integer[]                |           |          | 
 finished                 | integer[]                |           |          | 
 last_email_notification  | timestamp with time zone |           | not null | 
 last_in_app_notification | timestamp with time zone |           | not null | 
 last_sms_notification    | timestamp with time zone |           | not null | 
Indexes:
    "user_stats_pkey" PRIMARY KEY, btree (email)
    "user_stats_created_at_idx" btree (created_at)
    "user_stats_last_email_notification_idx" btree (last_email_notification)
    "user_stats_last_in_app_notification_idx" btree (last_in_app_notification)
    "user_stats_last_sms_notification_idx" btree (last_sms_notification)
    "user_stats_last_visited_at_idx" btree (last_visited_at)
    "user_stats_last_watched_at_idx" btree (last_watched_at)
    "user_stats_recent_watched_idx" gin (recent_watched)
    "user_stats_started_but_not_finished_idx" gin (started_but_not_finished)
    "user_stats_viewed_but_not_started_idx" gin (viewed_but_not_started)

user_stats=> 
```
