function get<Typ, Schlüssel extends keyof Typ>(element: Typ, schlüssel: Schlüssel) {
  return element[schlüssel];
}

type Mensch = {
  name: String;
};

const mensch: Mensch = {
  name: 'Name',
};

get(mensch, 'nichtName');
