type Box<T> = {
  value: T;
};

function unbox<T>(box: T | Box<T>): T {
  return box.value;
}

const box = { value: 42 };
const value = unbox(box);
//    ^? V
