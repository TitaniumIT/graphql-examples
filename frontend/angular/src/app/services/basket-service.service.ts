import { Injectable } from '@angular/core';
import { FieldFunctionOptions } from '@apollo/client/cache/inmemory/policies';
import { makeVar } from "@apollo/client/cache/inmemory/reactiveVars";
import { Reference } from '@apollo/client/cache/inmemory/types';
import { ProductFieldPolicy } from '../models/generated-clientHelpers';
import { BuyProductMutationService, ProductView } from '../models/generated-operations';


@Injectable({
  providedIn: 'root'
})
export class BasketService {

  private static orderedProducts = makeVar([""]);

  constructor(private buyProductMutation: BuyProductMutationService) {
  }

  public buyProduct(product: ProductView) {
    BasketService.orderedProducts([...BasketService.orderedProducts(), product.id]);

    this.buyProductMutation.mutate({productId:product.id, customerId:sessionStorage.getItem("basketId")! },
        { 
          optimisticResponse: {
            buy: {
              id: product.id,
              name: product.name!,
              inStock: product.inStock - 1,
              inBasket: true,
              actionsAllowed: [ "Processing"],
              inTransit: null,
              __typename: "Product"
            }
          }
        })
        .subscribe((result)=>{   console.log("Ordering 3:",result);});
   }

  public updateBasketWithId(productId: string) {
    BasketService.orderedProducts([...BasketService.orderedProducts(), productId]);
  }

  public static ProductLocalFields(): ProductFieldPolicy {
    return {
      inBasket: {
        read: (_, options) => {
          return BasketService.orderedProducts().includes(options.readField("id") as string);
        }
      },
      nrOrderd: {
        read: (_, options) => {
          return BasketService.orderedProducts().filter(x => x == options.readField("id") as string).length;
        }
      },
      nrInTransit: { read: (_, options) => this.CountState("InTransit",options) },
      nrDeliverd: { read: (_, options) => this.CountState("Deliverd",options)   },
      nrCancelled: { read: (_, options) => this.CountState("Cancelled",options) }
    }
  } 


  private static  CountState(state:string,options:FieldFunctionOptions) : number{
    let inTransit = options.readField({ fieldName: "productsInTransit", args: options.variables! }) as Reference[];
          if (inTransit)
            return inTransit.filter(x => options.readField("state", x) == state).length;
          else
            return 0;
  }

}

