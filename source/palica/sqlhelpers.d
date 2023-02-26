module palica.sqlhelpers;
import etc.c.sqlite3;

struct PreparedStatement
{
    ~this()
    {
        sqlite3_finalize(pstmt);
    }
    
private:
    sqlite3_stmt * pstmt;
}