import { Component, afterNextRender } from '@angular/core';
import { Footer } from '../footer/footer';

@Component({
  selector: 'app-about',
  imports: [Footer],
  templateUrl: './about.html',
  styleUrl: './about.scss'
})
export class About {
  constructor() {
    afterNextRender(() => {
      // This code runs only in the browser, after the next render cycle
      console.log('Window object is safe to use here:', window.location.href);
    });
  }
}
