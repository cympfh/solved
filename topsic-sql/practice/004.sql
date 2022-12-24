WITH FUCK1 AS (
    SELECT
        AGE_CODE,
        SUM(TARGET_POP) AS TARGET_POP
    FROM SLEEP_TIME_DTL
    WHERE TIME_CODE = 120
    GROUP BY AGE_CODE
),

FUCK2 AS (
    SELECT
        AGE_CODE,
        SUM(TARGET_POP) AS TARGET_POP
    FROM SLEEP_TIME_DTL
    WHERE TIME_CODE = 130
    GROUP BY AGE_CODE
),

FUCK3 AS (
    SELECT
        AGE_CODE,
        SUM(TARGET_POP) AS TARGET_POP
    FROM SLEEP_TIME_DTL
    WHERE TIME_CODE = 140
    GROUP BY AGE_CODE
),

FUCK4 AS (
    SELECT
        AGE_CODE,
        SUM(TARGET_POP) AS TARGET_POP
    FROM SLEEP_TIME_DTL
    WHERE TIME_CODE = 150
    GROUP BY AGE_CODE
),

FUCK5 AS (
    SELECT
        AGE_CODE,
        SUM(TARGET_POP) AS TARGET_POP
    FROM SLEEP_TIME_DTL
    WHERE TIME_CODE = 160
    GROUP BY AGE_CODE
),

FUCK6 AS (
    SELECT
        AGE_CODE,
        SUM(TARGET_POP) AS TARGET_POP
    FROM SLEEP_TIME_DTL
    WHERE TIME_CODE = 170
    GROUP BY AGE_CODE
),

FUCK7 AS (
    SELECT
        AGE_CODE,
        SUM(TARGET_POP) AS TARGET_POP
    FROM SLEEP_TIME_DTL
    WHERE TIME_CODE = 180
    GROUP BY AGE_CODE
)

SELECT
    AGE_GRP.AGE_NAME AS `年齢階層`,
    FUCK1.TARGET_POP AS `5時間未満`,
    FUCK2.TARGET_POP AS `5時間以上6時間未満`,
    FUCK3.TARGET_POP AS `6時間以上7時間未満`,
    FUCK4.TARGET_POP AS `7時間以上8時間未満`,
    FUCK5.TARGET_POP AS `8時間以上9時間未満`,
    FUCK6.TARGET_POP AS `9時間以上`,
    FUCK7.TARGET_POP AS `不詳`
FROM AGE_GRP
INNER JOIN FUCK1 ON FUCK1.AGE_CODE = AGE_GRP.AGE_CODE
INNER JOIN FUCK2 ON FUCK2.AGE_CODE = AGE_GRP.AGE_CODE
INNER JOIN FUCK3 ON FUCK3.AGE_CODE = AGE_GRP.AGE_CODE
INNER JOIN FUCK4 ON FUCK4.AGE_CODE = AGE_GRP.AGE_CODE
INNER JOIN FUCK5 ON FUCK5.AGE_CODE = AGE_GRP.AGE_CODE
INNER JOIN FUCK6 ON FUCK6.AGE_CODE = AGE_GRP.AGE_CODE
INNER JOIN FUCK7 ON FUCK7.AGE_CODE = AGE_GRP.AGE_CODE
ORDER BY
    AGE_GRP.AGE_CODE ASC;