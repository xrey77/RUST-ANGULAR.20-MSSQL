import { Component, afterNextRender } from '@angular/core';
import { Footer } from '../footer/footer';

@Component({
  selector: 'app-contact',
  imports: [Footer],
  templateUrl: './contact.html',
  styleUrl: './contact.scss'
})
export class Contact {
  constructor() {
    afterNextRender(() => {
      // This code runs only in the browser, after the next render cycle
      console.log('Window object is safe to use here:', window.location.href);
    });
  }

}
