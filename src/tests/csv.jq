def tocsv:
	["Cusip","Issue Date","Maturity Date", "Term", "Reopening?", "Bid To Cover"],
	(.[]|[.cusip, .issueDate, .maturityDate, .term, .reopening, .bidToCoverRatio])
	|@csv;

tocsv
	
