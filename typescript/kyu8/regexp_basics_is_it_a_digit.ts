interface String {
  // Necessary, keep it here.
  digit(): boolean;
}

String.prototype.digit = function (this: any): boolean {
  return /^[0-9]$/.test(this);
};
