import { useState, useEffect } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { Menu, X, Terminal, Globe } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import clsx from 'clsx';
import { useTranslation } from 'react-i18next';

const Navbar = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [scrolled, setScrolled] = useState(false);
  const location = useLocation();
  const { t, i18n } = useTranslation();

  const navLinks = [
    { name: t('nav.home'), path: '/' },
    { name: t('nav.features'), path: '/features' },
    { name: t('nav.about'), path: '/about' },
    { name: t('nav.contact'), path: '/contact' },
  ];

  const toggleLanguage = () => {
    const newLang = i18n.language.startsWith('zh') ? 'en' : 'zh';
    i18n.changeLanguage(newLang);
  };

  useEffect(() => {
    const handleScroll = () => {
      setScrolled(window.scrollY > 20);
    };
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  // Close mobile menu on route change
  useEffect(() => {
    setIsOpen(false);
  }, [location]);

  return (
    <nav
      className={clsx(
        'fixed top-0 left-0 right-0 z-50 transition-all duration-300 border-b border-transparent',
        scrolled ? 'bg-background/80 backdrop-blur-md border-white/10 py-4' : 'bg-transparent py-6'
      )}
    >
      <div className="container mx-auto px-6 flex items-center justify-between">
        <Link to="/" className="flex items-center space-x-2 group">
          <div className="relative">
            <Terminal className="w-8 h-8 text-accent-cyan group-hover:text-accent-pink transition-colors duration-300" />
            <div className="absolute inset-0 bg-accent-cyan/20 blur-lg rounded-full group-hover:bg-accent-pink/20 transition-colors duration-300" />
          </div>
          <span className="text-2xl font-display font-bold tracking-wider text-white">
            MI7<span className="text-accent-cyan group-hover:text-accent-pink transition-colors duration-300">Soft</span>
          </span>
        </Link>

        {/* Desktop Navigation */}
        <div className="hidden md:flex items-center space-x-8">
          {navLinks.map((link) => (
            <Link
              key={link.path}
              to={link.path}
              className={clsx(
                'text-sm font-medium tracking-wide transition-colors duration-300 hover:text-accent-cyan relative group',
                location.pathname === link.path ? 'text-accent-cyan' : 'text-gray-300'
              )}
            >
              {link.name}
              <span className={clsx(
                "absolute -bottom-1 left-0 w-0 h-0.5 bg-accent-cyan transition-all duration-300 group-hover:w-full",
                location.pathname === link.path ? "w-full" : "w-0"
              )} />
            </Link>
          ))}
          
          <button
            onClick={toggleLanguage}
            className="text-gray-300 hover:text-accent-cyan transition-colors"
            aria-label="Toggle Language"
          >
            <Globe size={20} />
          </button>

          <Link
            to="/contact"
            className="px-6 py-2 bg-accent-cyan/10 border border-accent-cyan/50 text-accent-cyan hover:bg-accent-cyan hover:text-black transition-all duration-300 rounded font-medium text-sm tracking-wide"
          >
            {t('nav.getStarted')}
          </Link>
        </div>

        {/* Mobile Menu Button */}
        <div className="flex items-center space-x-4 md:hidden">
            <button
                onClick={toggleLanguage}
                className="text-gray-300 hover:text-accent-cyan transition-colors"
              >
                <Globe size={20} />
            </button>
            <button
              className="text-white hover:text-accent-cyan transition-colors"
              onClick={() => setIsOpen(!isOpen)}
            >
              {isOpen ? <X size={24} /> : <Menu size={24} />}
            </button>
        </div>
      </div>

      {/* Mobile Navigation */}
      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: 'auto' }}
            exit={{ opacity: 0, height: 0 }}
            className="md:hidden bg-background/95 backdrop-blur-xl border-b border-white/10 overflow-hidden"
          >
            <div className="container mx-auto px-6 py-8 flex flex-col space-y-4">
              {navLinks.map((link) => (
                <Link
                  key={link.path}
                  to={link.path}
                  className={clsx(
                    'text-lg font-medium transition-colors hover:text-accent-cyan',
                    location.pathname === link.path ? 'text-accent-cyan' : 'text-gray-300'
                  )}
                >
                  {link.name}
                </Link>
              ))}
              <Link
                to="/contact"
                className="inline-block text-center px-6 py-3 bg-accent-cyan text-black font-bold rounded hover:bg-accent-cyan/90 transition-colors"
              >
                {t('nav.getStarted')}
              </Link>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </nav>
  );
};

export default Navbar;
