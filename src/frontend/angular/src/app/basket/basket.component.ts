import { Component, OnInit } from '@angular/core';
import { Subscription } from 'rxjs';
import * as uuid from 'uuid';
import { ToastrService } from 'ngx-toastr';
import { BasketEventsSubscriptionService, BasketView, GetBasketProductsQueryService } from '../models/generated-operations';


@Component({
  selector: 'app-basket',
  templateUrl: './basket.component.html',
  styleUrls: ['./basket.component.sass']
})
export class BasketComponent implements OnInit {

  inBasket: BasketView[] = [];
  customerId: string;

  private querySubscription!: Subscription;

  constructor(private toastr: ToastrService, private getBasket: GetBasketProductsQueryService, private basketEvents: BasketEventsSubscriptionService) {
    this.customerId = "test-customer@local.home";
  }
  ngOnDestroy(): void {
    this.querySubscription.unsubscribe();
  }

  ngOnInit(): void {

   
    sessionStorage.setItem("basketId", this.customerId);

    this.basketEvents.subscribe({ customerId: this.customerId })
      .subscribe(result => {
        if (result.data?.statusChanged!.state == 'Deliverd') {
          this.toastr.success(`${result.data.statusChanged.name}  has been deliverd`);
        }
        if (result.data?.statusChanged!.state == 'Cancelled') {
          this.toastr.warning(`${result.data.statusChanged.name}  has been cancelled`);
        }
      });

    this.querySubscription =
      this.getBasket.watch(
        { customerId: this.customerId },
        {
          fetchPolicy: 'cache-only',
          partialRefetch: true, returnPartialData: true
        })
        .valueChanges.subscribe({
          next: result => {
            console.log("getBasket: ", result);
            if (result.data && result.data.inBasketProducts) {
              this.inBasket = result?.data?.inBasketProducts?.items?.filter(x => x != null && x.inBasket) as BasketView[];
            }
          }, error: (result) => {
            console.log("error", result)
          }
        });
  }
}
