-- Add migration script here

CREATE TABLE dbo.products (
	id int IDENTITY(1,1) NOT NULL,
	alertstocks int DEFAULT 0 NULL,
	category nvarchar(4000) COLLATE SQL_Latin1_General_CP1_CI_AS NULL,
	costprice decimal(10,0) DEFAULT 0 NULL,
	created_at datetime2(6) NOT NULL,
	criticalstocks int DEFAULT 0 NULL,
	descriptions nvarchar(4000) COLLATE SQL_Latin1_General_CP1_CI_AS NOT NULL,
	productpicture nvarchar(4000) COLLATE SQL_Latin1_General_CP1_CI_AS NULL,
	qty int DEFAULT 0 NULL,
	saleprice decimal(10,0) DEFAULT 0 NULL,
	sellprice decimal(10,0) DEFAULT 0 NULL,
	unit nvarchar(4000) COLLATE SQL_Latin1_General_CP1_CI_AS NULL,
	updated_at datetime2(6) NOT NULL,
	CONSTRAINT PK__products__3213E83FF56FD62F PRIMARY KEY (id)
);
 CREATE  UNIQUE NONCLUSTERED INDEX index_products_on_descriptions ON dbo.products (  descriptions ASC  )  
	 WITH (  PAD_INDEX = OFF ,FILLFACTOR = 100  ,SORT_IN_TEMPDB = OFF , IGNORE_DUP_KEY = OFF , STATISTICS_NORECOMPUTE = OFF , ONLINE = OFF , ALLOW_ROW_LOCKS = ON , ALLOW_PAGE_LOCKS = ON  )
	 ON [PRIMARY ] ;