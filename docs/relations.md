Relations
=========


Every ORM must have relations. There are 3 types (one is artificial):
    1:N - ForeignKey
    1:1 - OneToOne (special type of ForeignKey with unique index)
    M:N - ManyToMany
    
First I will implement just ForeignKey since the other are just modifications(it will be hard)

ForeignKey will be defined as Option<Model>. This means that it will be nullable and there is no need
to add additional null/non null magic.
ForignKey will have custom annotation attribute nullable which means that NULL value is allowed. Default
wil be nullable=False.

During the initialization of ForeignKey(source code generation) I have to set there information about 
model fro the other side. Best it will be to store closure method to model_options (we can then read what we want).

