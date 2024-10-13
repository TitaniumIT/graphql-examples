
using System.Security.Claims;
using System.Security.Principal;

namespace GraphQLServer.helpers;

public class ManagerUser : ClaimsPrincipal, IIdentity
{
    public override bool IsInRole(string role)
    {
        return role == "Manager";
    }

    public override IIdentity? Identity => this;

    public string? AuthenticationType => "none";

    public bool IsAuthenticated => true;

    public string? Name => "Manager";
}


