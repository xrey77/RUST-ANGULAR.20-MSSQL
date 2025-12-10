-- Add migration script here

CREATE TABLE dbo.users (
	id int IDENTITY(1,1) NOT NULL,
	created_at datetime2(6) NOT NULL,
	email nvarchar(4000) COLLATE SQL_Latin1_General_CP1_CI_AS NOT NULL,
	firstname nvarchar(4000) COLLATE SQL_Latin1_General_CP1_CI_AS NULL,
	isactivated int DEFAULT 1 NULL,
	isblocked int DEFAULT 0 NULL,
	lastname nvarchar(4000) COLLATE SQL_Latin1_General_CP1_CI_AS NULL,
	mailtoken int DEFAULT 0 NULL,
	mobile nvarchar(4000) COLLATE SQL_Latin1_General_CP1_CI_AS NULL,
	password_digest nvarchar(4000) COLLATE SQL_Latin1_General_CP1_CI_AS NULL,
	qrcodeurl text NULL,
	roles nvarchar(4000) COLLATE SQL_Latin1_General_CP1_CI_AS DEFAULT N'ROLE_USER' NOT NULL,
	secret text NULL,
	updated_at datetime2(6) NOT NULL,
	userpic nvarchar(4000) COLLATE SQL_Latin1_General_CP1_CI_AS DEFAULT N'pix.png' NOT NULL,
	username nvarchar(255) COLLATE Latin1_General_CS_AS NOT NULL,
	CONSTRAINT PK__users__3213E83F88C81670 PRIMARY KEY (id)
);
 CREATE  UNIQUE NONCLUSTERED INDEX index_users_on_email ON dbo.users (  email ASC  )  
	 WITH (  PAD_INDEX = OFF ,FILLFACTOR = 100  ,SORT_IN_TEMPDB = OFF , IGNORE_DUP_KEY = OFF , STATISTICS_NORECOMPUTE = OFF , ONLINE = OFF , ALLOW_ROW_LOCKS = ON , ALLOW_PAGE_LOCKS = ON  )
	 ON [PRIMARY ] ;
 CREATE NONCLUSTERED INDEX index_users_on_username ON dbo.users (  username ASC  )  
	 WITH (  PAD_INDEX = OFF ,FILLFACTOR = 100  ,SORT_IN_TEMPDB = OFF , IGNORE_DUP_KEY = OFF , STATISTICS_NORECOMPUTE = OFF , ONLINE = OFF , ALLOW_ROW_LOCKS = ON , ALLOW_PAGE_LOCKS = ON  )
	 ON [PRIMARY ] ;