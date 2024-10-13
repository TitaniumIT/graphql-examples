namespace GraphQLServer.Models.ValueTypes;

public record EmailAddress
{
    private readonly string? _address;

    public EmailAddress(string strValue)
    {
        Address = strValue;
    }

    public string Address 
    {
        get => _address!;
        init
        {
            if(value == null) throw new ArgumentNullException(nameof(Address));
            if (System.Net.Mail.MailAddress.TryCreate(value, out _))
            {
                _address = value;
            }
            else
            {
                throw new FormatException($"EmailAddress with address {value} is invalid");
            }
        }
    }
}
