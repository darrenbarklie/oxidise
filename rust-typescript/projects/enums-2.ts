type Custom = {
  age: number;
  name: string;
};

type Item = number | string | Custom;

function append(items: Item[]) {
  items.push("Hello FEM");
}

const arrayItems: Item[] = [1, "two", { age: 123, name: "Jason" }];
console.log(arrayItems);
append(arrayItems);
console.log(arrayItems);

const arrayNumbers: number[] = [4, 5, 6];
console.log(arrayNumbers);
append(arrayNumbers);
console.log(arrayNumbers);
