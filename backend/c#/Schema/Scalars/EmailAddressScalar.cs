using GraphQL.Types;
using GraphQLServer.Models.ValueTypes;

namespace GraphQLServer.Schema.Scalars
{
    public class EmailAddressScalar : ScalarGraphType
    {
        public override object? ParseValue(object? value)
        {
            if (value is string strValue) { return new EmailAddress(strValue); }
            if (value is EmailAddress email) { return email.Address; }
            if (value == null) { return null; }

            throw new FormatException($"Provided value {value} is not convertable to EmailAdress");
        }
    }
}
