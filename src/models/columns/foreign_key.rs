use super::super::Model;

/*
Foreign key should have functionality for None values, and in column annotation there should be option "blank"
 */
type ForeignKey = Option<Model>;
