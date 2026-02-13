import { Terminal, Github, Twitter, Linkedin } from 'lucide-react';
import { Link } from 'react-router-dom';

const Footer = () => {
  return (
    <footer className="bg-surface-dark border-t border-white/10 pt-16 pb-8">
      <div className="container mx-auto px-6">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-12 mb-12">
          <div className="col-span-1 md:col-span-1">
            <Link to="/" className="flex items-center space-x-2 mb-4 group">
              <Terminal className="w-6 h-6 text-accent-cyan group-hover:text-accent-pink transition-colors duration-300" />
              <span className="text-xl font-display font-bold text-white">
                MI7<span className="text-accent-cyan group-hover:text-accent-pink transition-colors duration-300">Soft</span>
              </span>
            </Link>
            <p className="text-gray-400 text-sm leading-relaxed">
              Empowering Linux systems with high-performance, secure, and reliable daemon management solutions.
            </p>
          </div>

          <div>
            <h3 className="font-display font-bold text-white mb-4">Product</h3>
            <ul className="space-y-2">
              <li><Link to="/features" className="text-gray-400 hover:text-accent-cyan transition-colors text-sm">Features</Link></li>
              <li><Link to="/features" className="text-gray-400 hover:text-accent-cyan transition-colors text-sm">Integrations</Link></li>
              <li><Link to="/features" className="text-gray-400 hover:text-accent-cyan transition-colors text-sm">Performance</Link></li>
            </ul>
          </div>

          <div>
            <h3 className="font-display font-bold text-white mb-4">Company</h3>
            <ul className="space-y-2">
              <li><Link to="/about" className="text-gray-400 hover:text-accent-cyan transition-colors text-sm">About Us</Link></li>
              <li><Link to="/about" className="text-gray-400 hover:text-accent-cyan transition-colors text-sm">Careers</Link></li>
              <li><Link to="/contact" className="text-gray-400 hover:text-accent-cyan transition-colors text-sm">Contact</Link></li>
            </ul>
          </div>

          <div>
            <h3 className="font-display font-bold text-white mb-4">Connect</h3>
            <div className="flex space-x-4">
              <a href="#" className="text-gray-400 hover:text-accent-cyan transition-colors">
                <Github size={20} />
              </a>
              <a href="#" className="text-gray-400 hover:text-accent-cyan transition-colors">
                <Twitter size={20} />
              </a>
              <a href="#" className="text-gray-400 hover:text-accent-cyan transition-colors">
                <Linkedin size={20} />
              </a>
            </div>
          </div>
        </div>

        <div className="border-t border-white/5 pt-8 flex flex-col md:flex-row justify-between items-center">
          <p className="text-gray-500 text-sm">
            &copy; {new Date().getFullYear()} MI7 Soft. All rights reserved.
          </p>
          <div className="flex space-x-6 mt-4 md:mt-0">
            <a href="#" className="text-gray-500 hover:text-white text-sm transition-colors">Privacy Policy</a>
            <a href="#" className="text-gray-500 hover:text-white text-sm transition-colors">Terms of Service</a>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
