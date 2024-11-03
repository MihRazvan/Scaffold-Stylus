import React, { useState } from 'react';
import { Book, Edit3 } from 'lucide-react';

const ContractPlayground = () => {
    const [activeTab, setActiveTab] = useState<'read' | 'write'>('read');

    const readFunctions = [
        {
            name: 'balanceOf',
            description: 'Returns the token balance of an account',
            params: [{ name: 'Account Address', placeholder: '0x...' }]
        },
        {
            name: 'totalSupply',
            description: 'Returns the total token supply',
            params: []
        }
    ];

    const writeFunctions = [
        {
            name: 'transfer',
            description: 'Transfer tokens to another address',
            params: [
                { name: 'Recipient', placeholder: '0x...' },
                { name: 'Amount', placeholder: '0.0' }
            ]
        },
        {
            name: 'approve',
            description: 'Approve spending of tokens by another address',
            params: [
                { name: 'Spender', placeholder: '0x...' },
                { name: 'Amount', placeholder: '0.0' }
            ]
        }
    ];

    const functions = activeTab === 'read' ? readFunctions : writeFunctions;

    return (
        <div>
            {/* Tabs */}
            <div className="flex space-x-4 mb-6">
                <button
                    onClick={() => setActiveTab('read')}
                    className={`flex items-center gap-2 px-4 py-2 rounded-lg transition-colors ${activeTab === 'read'
                            ? 'bg-blue-500/20 text-blue-400 border border-blue-500/20'
                            : 'text-gray-400 hover:text-gray-300'
                        }`}
                >
                    <Book className="w-4 h-4" />
                    Read Functions
                </button>
                <button
                    onClick={() => setActiveTab('write')}
                    className={`flex items-center gap-2 px-4 py-2 rounded-lg transition-colors ${activeTab === 'write'
                            ? 'bg-purple-500/20 text-purple-400 border border-purple-500/20'
                            : 'text-gray-400 hover:text-gray-300'
                        }`}
                >
                    <Edit3 className="w-4 h-4" />
                    Write Functions
                </button>
            </div>

            {/* Function Cards */}
            <div className="space-y-4">
                {functions.map((func) => (
                    <div key={func.name} className="p-4 rounded-lg bg-gray-800/50 border border-gray-700">
                        <div className="flex items-start justify-between mb-4">
                            <div>
                                <h3 className="text-lg font-medium text-gray-200">{func.name}</h3>
                                <p className="text-sm text-gray-400">{func.description}</p>
                            </div>
                            <span className={`px-2 py-1 text-xs font-medium rounded-full ${activeTab === 'read'
                                    ? 'bg-blue-500/20 text-blue-400 border border-blue-500/20'
                                    : 'bg-purple-500/20 text-purple-400 border border-purple-500/20'
                                }`}>
                                {activeTab === 'read' ? 'Read' : 'Write'}
                            </span>
                        </div>

                        <div className="space-y-4">
                            {func.params.map((param, index) => (
                                <div key={index}>
                                    <label className="block text-sm font-medium text-gray-300 mb-2">
                                        {param.name}
                                    </label>
                                    <input
                                        type="text"
                                        className="w-full px-3 py-2 rounded-lg bg-gray-900/50 border border-gray-700 text-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500/50"
                                        placeholder={param.placeholder}
                                    />
                                </div>
                            ))}

                            <button className={`w-full px-4 py-2 rounded-lg ${activeTab === 'read'
                                    ? 'bg-blue-500 hover:bg-blue-600'
                                    : 'bg-purple-500 hover:bg-purple-600'
                                } text-white font-medium transition-colors`}>
                                {activeTab === 'read' ? 'Query' : 'Execute'}
                            </button>

                            <div className="mt-4 p-3 rounded-lg bg-gray-900/50 border border-gray-700">
                                <div className="text-sm text-gray-400 mb-1">Result:</div>
                                <div className="font-mono text-gray-200">0.0</div>
                            </div>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
}

export default ContractPlayground;