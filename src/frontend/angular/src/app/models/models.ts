import { makeVar } from "@apollo/client/cache/inmemory/reactiveVars";
import { gql } from "apollo-angular";
import { EmptyObject } from "apollo-angular/types";



export const orderedProducts = makeVar([""]);
