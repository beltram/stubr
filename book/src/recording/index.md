# Recording

Writing stubs by hand can be... well, painful. More often than not, you already have an existing app ready and you just
want to mock it for consumers. Or you are from those who prefer writing code before tests. Or you are lazy 😅.  

All those are valid reasons and in order to help you with that, you can record real http traffic and turn it into json
stub files. If you are not at all working on Rust projects, you can [record using the cli](cli.md) to capture traffic
from for example a real application in production written in Java. If you are in a Rust project and use a http client
library, you can record its traffic in tests for [actix](actix.md), [isahc](isahc.md) or [reqwest](reqwest.md). And
if your favorite http client is not is this list you can still record using a [standalone](standalone.md) recorder. 
Recording in a Rust consumer is mostly about getting faster, especially with "fat" endpoints with hundreds of fields. 