import React from 'react';
import { App } from './App';
import { createRoot } from 'react-dom/client';

import 'swiper/css';
import 'swiper/css/navigation';
import 'swiper/css/pagination';
import './styles.css';

const ROOT = createRoot(document.querySelector('#ROOT') as HTMLDivElement);

ROOT.render(<App isLoggedIn={!!document.cookie.match(/alfred_is_logged_in=true/)?.length} />);
