# Ars (stands for auction result)

ars is a command line tool that can show the results of treasury auctions. The´se results can move market when they are either very bad (tails) or are received as very good (stop through). You can get the results for bonds that were auctioned let's say in the 153 days and were in the 20yr tenor by:

```console
foo@bar:~$ ars latest --sectype=bond --tenor=20y --days=153
foo
```
which gives the following results:

```console
foo@bar:~$
+------------------+-----------+-----------+---------------+------------+---------------+--------------+-----------+-----------+-------------+------------+---------------+
| Security Term    | CUSIP     | Reopening | Security Type | Issue Date | Maturity Date | Bid To Cover | Dealers % | Directs % | Indirects % | High Yield | Interest Rate |
+------------------+-----------+-----------+---------------+------------+---------------+--------------+-----------+-----------+-------------+------------+---------------+
| 19-Year 11-Month | 912810TZ1 |    Yes    | Bond          | 04/01/2024 | 02/15/2044    |         2.79 |     9.35% |    17.16% |      73.49% |     4.542% |        4.500% |
+------------------+-----------+-----------+---------------+------------+---------------+--------------+-----------+-----------+-------------+------------+---------------+
| 20-Year          | 912810TZ1 |    No     | Bond          | 02/29/2024 | 02/15/2044    |         2.39 |    21.21% |    19.71% |      59.08% |     4.595% |        4.500% |
+------------------+-----------+-----------+---------------+------------+---------------+--------------+-----------+-----------+-------------+------------+---------------+
| 19-Year 10-Month | 912810TW8 |    Yes    | Bond          | 01/31/2024 | 11/15/2043    |         2.53 |    17.33% |    20.51% |      62.16% |     4.423% |        4.750% |
+------------------+-----------+-----------+---------------+------------+---------------+--------------+-----------+-----------+-------------+------------+---------------+
| 19-Year 11-Month | 912810TW8 |    Yes    | Bond          | 01/02/2024 | 11/15/2043    |         2.55 |    12.89% |    20.71% |      66.40% |     4.213% |        4.750% |
+------------------+-----------+-----------+---------------+------------+---------------+--------------+-----------+-----------+-------------+------------+---------------+
| 20-Year          | 912810TW8 |    No     | Bond          | 11/30/2023 | 11/15/2043    |         2.58 |     9.51% |    16.51% |      73.98% |     4.780% |        4.750% |
+------------------+-----------+-----------+---------------+------------+---------------+--------------+-----------+-----------+-------------+------------+---------------+
```

You have also the possiblity to display the results in a vertical format if you apple the --vertical (-E) parameter:

```console
foo@bar:~$ ars -E latest --sectype=bond --tenor=20y --days=153

 Security Term:  19-Year 11-Month
 CUSIP           912810TZ1
 Reopening:      Yes
 Security Type:  Bond
 Issue Date:     04/01/2024
 Maturity Date:  02/15/2044
 Bid To Cover:   2.79
 Dealers %:      9.35%
 Directs %:      17.16%
 Indirects %:    73.49%
 High Yield:     4.542%
 Interest Rate:  4.500%

 Security Term:  20-Year
 CUSIP           912810TZ1
 Reopening:      No
 Security Type:  Bond
 Issue Date:     02/29/2024
 Maturity Date:  02/15/2044
 Bid To Cover:   2.39
 Dealers %:      21.21%
 Directs %:      19.71%
 Indirects %:    59.08%
 High Yield:     4.595%
 Interest Rate:  4.500%
,,,
```
