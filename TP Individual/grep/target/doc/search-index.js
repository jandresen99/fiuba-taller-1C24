var searchIndex = new Map(JSON.parse('[\
["grep",{"doc":"","t":"CCCCCCCFONNNNNNNOONNNNFNNNNNNNNNNNNNPPPPPGPPNNNNNNNNNNNPPPPPGNNNNNNNNNNNFNNNNNNNONNNNOPPPPGPNNNNNNNNNNNNHH","n":["evaluated_step","regex","regex_class","regex_rep","regex_step","regex_val","utils","EvaluatedStep","backtrackable","borrow","borrow_mut","clone","clone_into","fmt","from","into","match_size","step","to_owned","try_from","try_into","type_id","Regex","borrow","borrow_mut","clone","clone_into","fmt","from","into","new","test","to_owned","try_from","try_into","type_id","Alphabetic","Alphanumeric","Digit","Lowercase","Punctuation","RegexClass","Uppercase","Whitespace","borrow","borrow_mut","clone","clone_into","fmt","from","into","to_owned","try_from","try_into","type_id","Any","Exact","Last","Optional","Range","RegexRep","borrow","borrow_mut","clone","clone_into","fmt","from","into","to_owned","try_from","try_into","type_id","RegexStep","borrow","borrow_mut","clone","clone_into","fmt","from","into","rep","to_owned","try_from","try_into","type_id","val","Allowed","Class","Literal","NotAllowed","RegexVal","Wildcard","borrow","borrow_mut","clone","clone_into","fmt","from","into","matches","to_owned","try_from","try_into","type_id","read_args","read_lines"],"q":[[0,"grep"],[7,"grep::evaluated_step"],[22,"grep::regex"],[36,"grep::regex_class"],[55,"grep::regex_rep"],[72,"grep::regex_step"],[86,"grep::regex_val"],[104,"grep::utils"],[106,"core::fmt"],[107,"core::fmt"],[108,"core::any"],[109,"std::io::error"],[110,"alloc::string"],[111,"alloc::vec"]],"d":["","","","","","","","Representa un paso de la regex que ya ha sido evaluado.","Representa un paso de la regex que ya ha sido evaluado.","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Representa un paso de la regex que ya ha sido evaluado.","Representa un paso de la regex que ya ha sido evaluado.","","","","","Esta estructura representa una regex","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Crea una regex utilizando la expression que recibe por …","Prueba la regex contra un string. Devuelve true si el …","","","","","Debe ser un caracter alphabetico (letras).","Debe ser un caracter alfanumerico (letras + numeros).","Debe ser un digito (numeros).","Debe ser una letra en minuscula.","Debe ser un simbolo de puntuacion.","Representa un tipo de repetición de clase a la cual debe …","Debe ser una letra en mayuscula.","Debe ser un espacio en blanco.","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","Se puede repetir varias veces","Se puede repetir una cantidad especifica de veces","Debe ser el final del valor","Puede existir o no","Se puede repetir dentro de un rango de veces","Un RegexStep representa un tipo de repetición que va a …","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","Esta estructura representa un paso de la regex a evaluar.","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Cada paso debe incluir un tipo de repeticion.","","","","","Cada paso debe incluir un tipo de valor contra el cual se …","Se busca que el valor se encuentre dentro de un vector de …","Se busca que el valor cumpla una serie de condiciones …","Se busca que el valor sea un caracter especifico.","Se busca que el valor NO se encuentre dentro de un vector …","Esta estructura representa un valor de la expression de …","Se busca que el valor sea cualquier caracter.","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Prueba si el valor recibido cumple con el tipo de RegexVal.","","","","","Lee los argumentos pasados por comando y devuelve una …","Lee el archivo indicado y devuelve una lista con cada linea"],"i":[0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,7,7,7,7,7,7,7,7,7,7,7,7,7,11,11,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,0,12,12,12,12,12,12,12,12,12,12,12,0,13,13,13,13,13,13,13,13,13,13,13,13,13,14,14,14,14,0,14,14,14,14,14,14,14,14,14,14,14,14,14,0,0],"f":[0,0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[1,1],[[-1,-2],2,[],[]],[[1,3],4],[-1,-1,[]],[-1,-2,[],[]],0,0,[-1,-2,[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],0,[-1,-2,[],[]],[-1,-2,[],[]],[7,7],[[-1,-2],2,[],[]],[[7,3],4],[-1,-1,[]],[-1,-2,[],[]],[8,[[5,[7,9]]]],[[7,8],[[5,[10,9]]]],[-1,-2,[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[11,11],[[-1,-2],2,[],[]],[[11,3],4],[-1,-1,[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[12,12],[[-1,-2],2,[],[]],[[12,3],4],[-1,-1,[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],0,[-1,-2,[],[]],[-1,-2,[],[]],[13,13],[[-1,-2],2,[],[]],[[13,3],4],[-1,-1,[]],[-1,-2,[],[]],0,[-1,-2,[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[14,14],[[-1,-2],2,[],[]],[[14,3],4],[-1,-1,[]],[-1,-2,[],[]],[[14,8],15],[-1,-2,[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[[],[[5,[[17,[16]],9]]]],[16,[[5,[[17,[16]],9]]]]],"c":[],"p":[[5,"EvaluatedStep",7],[1,"tuple"],[5,"Formatter",106],[8,"Result",106],[6,"Result",107],[5,"TypeId",108],[5,"Regex",22],[1,"str"],[5,"Error",109],[1,"bool"],[6,"RegexClass",36],[6,"RegexRep",55],[5,"RegexStep",72],[6,"RegexVal",86],[1,"usize"],[5,"String",110],[5,"Vec",111]],"b":[]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
