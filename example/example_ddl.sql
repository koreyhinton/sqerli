/*
    <--  GBook.Gid GBookLang.Gid GBookTitle.Gid
    -->> GBookTransl.Gid GBookWordPos.Gid
    <--  GBookWordPos.WordId
    -->> Word.WordId
*/
create table GBook (Gid text);
create table GBookLang (Gid text, Lang text);
create table GBookTransl (Gid text, TranslGid text);
create table GBookWordPos (Gid text, WordId integer, Pos integer);
create table Word (WordId integer primary key autoincrement, Word text);
create table GBookTitle (Gid text, title text);
