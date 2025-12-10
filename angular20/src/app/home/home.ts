import { NgbCarouselConfig, NgbCarouselModule } from '@ng-bootstrap/ng-bootstrap';
import { Component, afterNextRender } from '@angular/core';
import { Footer } from '../footer/footer';

@Component({
  selector: 'app-home',
  imports: [Footer,NgbCarouselModule],
  templateUrl: './home.html',
  styleUrl: './home.scss',
	providers: [NgbCarouselConfig],  
})

export class Home {
	// showNavigationArrows = false;
	// showNavigationIndicators = false;

	constructor(config: NgbCarouselConfig) {
		// customize default values of carousels used by this component tree
		config.interval = 10000;
		config.wrap = false;
		config.keyboard = false;
		config.pauseOnHover = false;
    // config.showNavigationArrows = true;
		// config.showNavigationIndicators = false;    
	}  

}

