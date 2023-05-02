unsafe DateTime? GetSystemExpirationDate() => *(long*)0x7ffe02c8 switch
{
    0 => null,
    var x => DateTime.FromFileTime(x)
};

var date = GetSystemExpirationDate();
Console.WriteLine(date);