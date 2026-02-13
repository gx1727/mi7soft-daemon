import { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import clsx from 'clsx';

interface CodeBlockProps {
  code: string;
  language?: string;
  className?: string;
}

const CodeBlock = ({ code, language = 'rust', className }: CodeBlockProps) => {
  const [displayedCode, setDisplayedCode] = useState('');
  
  useEffect(() => {
    let index = 0;
    const interval = setInterval(() => {
      setDisplayedCode((prev) => {
        if (index < code.length) {
          index++;
          return code.substring(0, index);
        }
        clearInterval(interval);
        return prev;
      });
    }, 20); // Typing speed

    return () => clearInterval(interval);
  }, [code]);

  return (
    <div className={clsx("bg-[#0d1117] rounded-lg overflow-hidden border border-white/10 font-mono text-sm", className)}>
      <div className="flex items-center px-4 py-2 bg-white/5 border-b border-white/5">
        <div className="flex space-x-2">
          <div className="w-3 h-3 rounded-full bg-red-500" />
          <div className="w-3 h-3 rounded-full bg-yellow-500" />
          <div className="w-3 h-3 rounded-full bg-green-500" />
        </div>
        <span className="ml-4 text-xs text-gray-500">{language}</span>
      </div>
      <div className="p-4 overflow-x-auto">
        <pre>
          <code className="text-gray-300">
            {displayedCode}
            <motion.span
              animate={{ opacity: [1, 0] }}
              transition={{ repeat: Infinity, duration: 0.8 }}
              className="inline-block w-2 h-4 bg-accent-cyan ml-1 align-middle"
            />
          </code>
        </pre>
      </div>
    </div>
  );
};

export default CodeBlock;
